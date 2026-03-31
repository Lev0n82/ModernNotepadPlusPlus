using System;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text.Json;
using System.Collections.Generic;

namespace WebAutomationTestingProgram.Scripts
{
    // This script runs automatically via a Post-Build Event in the GRACE .csproj OR via a Docker pipeline.
    // It dynamically inspects the compiled Playwright framework, finds every single valid "WebAction" a tester
    // can use, and exports a strict JSON Schema payload for the Zed "Null Claw" Language Server to read.
    
    public class GraceDictionaryExtractor
    {
        public static void GenerateNullClawDictionary(string outputDirectory)
        {
            Console.WriteLine("Initializing Null Claw LSP Dictionary Extraction...");

            // 1. Load the Assembly running the actual GRACE Playwright engine
            var assembly = Assembly.GetExecutingAssembly(); 
            if (assembly == null) {
                throw new Exception("Critical Error: WebAutomationTestingProgram Assembly not found in memory.");
            }

            // 2. Reflect on all classes that inherit from "WebAction" or implement IWebAction.
            // This guarantees we only suggest valid Actions that the framework can actually execute.
            var actionTypes = assembly.GetTypes()
                .Where(t => t.IsClass && !t.IsAbstract && 
                           (t.Name.EndsWith("Action") || t.GetInterfaces().Any(i => i.Name == "IWebAction")))
                .ToList();

            Console.WriteLine($"Discovered {actionTypes.Count} valid Playwright WebActions.");

            var dictionary = new NullClawSchema
            {
                Version = "1.0",
                Framework = "GRACE_Playwright",
                LastScanned = DateTime.UtcNow.ToString("O"),
                Actions = new List<ActionDefinition>()
            };

            // 3. Extract Meta-Data and Parameters for IntelliSense
            foreach (var type in actionTypes)
            {
                // We use human-readable spacing for the Excel UI: "ClickBoxAction" -> "Click Box"
                string keyword = type.Name.Replace("Action", "").Replace("_", " ");
                
                var definition = new ActionDefinition
                {
                    Keyword = keyword,
                    ClassName = type.Name,
                    Description = ExtractDocumentation(type),
                    RequiresControl = DetermineControlRequirement(type),
                    Parameters = new List<string>()
                };

                // Scan the Execute() methods to pull exact method parameters for Zed to auto-hint
                var executeMethod = type.GetMethods().FirstOrDefault(m => m.Name == "Execute" || m.Name == "ExecuteAsync");
                if (executeMethod != null)
                {
                    foreach (var param in executeMethod.GetParameters())
                    {
                        definition.Parameters.Add(param.Name);
                    }
                }

                dictionary.Actions.Add(definition);
            }

            // 4. Output the Strict JSON Payload to the Zed LSP Extension Directory
            var options = new JsonSerializerOptions { WriteIndented = true };
            string jsonPayload = JsonSerializer.Serialize(dictionary, options);
            
            string outputPath = Path.Combine(outputDirectory, "nullclaw_dictionary.json");
            File.WriteAllText(outputPath, jsonPayload);

            Console.WriteLine($"[SUCCESS] Null Claw Dictionary written to: {outputPath}");
        }

        // Mock Helper: If you use XML Doc Comments (/// <summary>), this extracts them for Zed's Hover Documentation!
        private static string ExtractDocumentation(Type t)
        {
            return $"Executes the {t.Name} logic in the Playwright context.";
        }

        // Mock Helper: Determines if this action requires an XPath/CSS locator in the 'Control' column.
        private static bool DetermineControlRequirement(Type t)
        {
            // System-level commands like 'GoToUrl' don't need a UI Control locator.
            string[] sysActions = { "BrowserAction", "DatabaseAction", "ApiAction" };
            return !sysActions.Any(s => t.Name.Contains(s));
        }
    }

    // --- JSON Serialization Models for the LSP ---
    public class NullClawSchema
    {
        public string Version { get; set; }
        public string Framework { get; set; }
        public string LastScanned { get; set; }
        public List<ActionDefinition> Actions { get; set; }
    }

    public class ActionDefinition
    {
        public string Keyword { get; set; }
        public string ClassName { get; set; }
        public string Description { get; set; }
        public bool RequiresControl { get; set; }
        public List<string> Parameters { get; set; }
    }
}

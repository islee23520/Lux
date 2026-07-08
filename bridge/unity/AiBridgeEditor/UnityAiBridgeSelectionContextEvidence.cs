using System;
using System.IO;
using System.Text;
using UnityEngine;

namespace Linalab.UnityAiBridge.Editor
{
    internal static class UnityAiBridgeSelectionContextEvidence
    {
        private const string SelectionContextFileName = "selection-context.json";
        private const string SelectionContextEventsFileName = "selection-context-events.jsonl";
        private const string SelectionContextCopiedEventType = "unity.selection_context_copied";

        internal static string ProjectRootOverrideForTests;

        internal static string ResolveProjectRoot()
        {
            if (ProjectRootOverrideForTests != null)
            {
                return ProjectRootOverrideForTests;
            }

            string projectRoot;
            string unavailableReason;
            if (!TryBuildUnityProjectRootFromDataPath(Application.dataPath, out projectRoot, out unavailableReason))
            {
                return string.Empty;
            }

            return projectRoot;
        }

        internal static bool TryBuildUnityProjectRootFromDataPath(string dataPath, out string projectRoot, out string unavailableReason)
        {
            projectRoot = string.Empty;
            unavailableReason = string.Empty;

            if (string.IsNullOrEmpty(dataPath))
            {
                unavailableReason = "Application.dataPath is empty; Unity project root is unavailable.";
                return false;
            }

            var assetsDirectory = new DirectoryInfo(dataPath);
            if (assetsDirectory.Parent == null)
            {
                unavailableReason = $"Application.dataPath has no parent directory: {dataPath}";
                return false;
            }

            projectRoot = assetsDirectory.Parent.FullName;
            return true;
        }

        internal static bool TryBuildPaths(string projectRoot, out string contextPath, out string contextEventsPath, out string unavailableReason)
        {
            contextPath = string.Empty;
            contextEventsPath = string.Empty;
            unavailableReason = string.Empty;

            if (string.IsNullOrEmpty(projectRoot))
            {
                unavailableReason = "Unity project root was unavailable; selection context evidence was not written.";
                return false;
            }

            var contextDirectory = Path.Combine(projectRoot, ".lux", "context");
            contextPath = Path.Combine(contextDirectory, SelectionContextFileName);
            contextEventsPath = Path.Combine(contextDirectory, SelectionContextEventsFileName);
            return true;
        }

        internal static bool TryWrite(UnityAiBridgeSelectionContextPayload payload, string projectRoot, bool logWarnings)
        {
            if (payload == null)
            {
                return false;
            }

            string contextPath;
            string contextEventsPath;
            string unavailableReason;
            if (!TryBuildPaths(projectRoot, out contextPath, out contextEventsPath, out unavailableReason))
            {
                MarkUnavailable(payload, unavailableReason, logWarnings);
                return false;
            }

            try
            {
                Directory.CreateDirectory(Path.GetDirectoryName(contextPath));
                payload.registrationStatus = "registered";
                payload.registrationUnavailableReason = string.Empty;
                payload.contextPath = contextPath;
                payload.contextEventsPath = contextEventsPath;

                File.WriteAllText(contextPath, JsonUtility.ToJson(payload, true), Encoding.UTF8);
                File.AppendAllText(contextEventsPath, BuildEventLine(payload, contextPath) + Environment.NewLine, Encoding.UTF8);
                return true;
            }
            catch (Exception ex)
            {
                var reason = $"Failed to write selection context evidence under {projectRoot}: {ex.GetType().Name}: {ex.Message}";
                MarkUnavailable(payload, reason, logWarnings);
                return false;
            }
        }

        private static string BuildEventLine(UnityAiBridgeSelectionContextPayload payload, string contextPath)
        {
            return JsonUtility.ToJson(new UnityAiBridgeSelectionContextCopiedEvent
            {
                schemaVersion = 1,
                eventType = SelectionContextCopiedEventType,
                capturedAtUtc = DateTime.UtcNow.ToString("o"),
                contextPath = contextPath,
                selectionCount = payload.selectionCount,
                summary = payload.summary
            });
        }

        private static void MarkUnavailable(UnityAiBridgeSelectionContextPayload payload, string reason, bool logWarning)
        {
            payload.registrationStatus = "unavailable";
            payload.registrationUnavailableReason = reason;
            payload.contextPath = string.Empty;
            payload.contextEventsPath = string.Empty;

            if (logWarning)
            {
                Debug.LogWarning("Lux selection context evidence unavailable: " + reason);
            }
        }
    }
}

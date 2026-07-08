using System;
using UnityEditor;
using UnityEngine;

namespace Linalab.UnityAiBridge.Editor
{
    public static class UnityAiBridgeSelectionContext
    {
        public static string BuildClipboardText(SerializedProperty highlightedProperty)
        {
            var payload = BuildPayload(highlightedProperty);
            return UnityAiBridgeSelectionContextClipboard.Format(payload);
        }

        internal static string BuildClipboardTextForProjectRoot(string projectRoot, SerializedProperty highlightedProperty)
        {
            var payload = BuildPayload(highlightedProperty);
            UnityAiBridgeSelectionContextEvidence.TryWrite(payload, projectRoot, false);
            return UnityAiBridgeSelectionContextClipboard.Format(payload);
        }

        internal static UnityAiBridgeSelectionContextPayload BuildPayload(SerializedProperty highlightedProperty)
        {
            var selectionAst = Ast.UnityAstSelectionReader.ReadSelection();
            var selectedObjects = UnityAiBridgeSelectionContextObjects.BuildSelectedObjectContexts(Selection.gameObjects);
            var highlighted = highlightedProperty == null ? null : UnityAiBridgeSelectionContextObjects.BuildHighlightedProperty(highlightedProperty);
            var summary = BuildSummary(selectedObjects, highlighted);

            return new UnityAiBridgeSelectionContextPayload
            {
                schemaVersion = 1,
                contextKind = "unity_selection_context",
                capturedAtUtc = DateTime.UtcNow.ToString("o"),
                summary = summary,
                registrationStatus = "unavailable",
                registrationUnavailableReason = "Selection context evidence has not been written to a Unity project .lux/context path.",
                contextPath = string.Empty,
                contextEventsPath = string.Empty,
                selectionCount = selectedObjects.Length,
                selectedObjects = selectedObjects,
                highlightedProperty = highlighted,
                selectionAst = selectionAst
            };
        }

        internal static void CopyPayloadToClipboard(UnityAiBridgeSelectionContextPayload payload)
        {
            var projectRoot = UnityAiBridgeSelectionContextEvidence.ResolveProjectRoot();
            UnityAiBridgeSelectionContextEvidence.TryWrite(payload, projectRoot, true);
            EditorGUIUtility.systemCopyBuffer = UnityAiBridgeSelectionContextClipboard.Format(payload);
        }

        private static string BuildSummary(UnityAiBridgeSelectedObjectContext[] selectedObjects, UnityAiBridgeHighlightedPropertyContext highlightedProperty)
        {
            if (highlightedProperty != null)
            {
                return $"Lux Unity selection context: property {highlightedProperty.propertyPath} on {highlightedProperty.targetName}.";
            }

            if (selectedObjects == null || selectedObjects.Length == 0)
            {
                return "Lux Unity selection context: no GameObject is selected.";
            }

            if (selectedObjects.Length == 1)
            {
                return $"Lux Unity selection context: {selectedObjects[0].hierarchyPath}.";
            }

            return $"Lux Unity selection context: {selectedObjects.Length} GameObjects selected.";
        }
    }
}

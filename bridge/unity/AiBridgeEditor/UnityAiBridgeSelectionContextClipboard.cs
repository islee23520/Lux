using System.Text;
using UnityEngine;

namespace Linalab.UnityAiBridge.Editor
{
    internal static class UnityAiBridgeSelectionContextClipboard
    {
        internal static string Format(UnityAiBridgeSelectionContextPayload payload)
        {
            var builder = new StringBuilder();
            builder.AppendLine(payload.summary);
            builder.AppendLine();
            builder.AppendLine("Selection:");

            if (payload.selectedObjects == null || payload.selectedObjects.Length == 0)
            {
                builder.AppendLine("- No GameObject selected.");
            }
            else
            {
                for (var i = 0; i < payload.selectedObjects.Length; i++)
                {
                    AppendSelectedObject(builder, payload.selectedObjects[i]);
                }
            }

            if (payload.highlightedProperty != null)
            {
                builder.Append("- Highlighted property: ");
                builder.Append(payload.highlightedProperty.propertyPath);
                builder.Append(" on ");
                builder.AppendLine(payload.highlightedProperty.targetName);
            }

            AppendRegistration(builder, payload);

            builder.AppendLine();
            builder.AppendLine("```json");
            builder.AppendLine(JsonUtility.ToJson(payload, true));
            builder.AppendLine("```");
            return builder.ToString();
        }

        private static void AppendSelectedObject(StringBuilder builder, UnityAiBridgeSelectedObjectContext selected)
        {
            builder.Append("- ");
            builder.Append(string.IsNullOrEmpty(selected.hierarchyPath) ? selected.name : selected.hierarchyPath);
            builder.Append(" (InstanceID ");
            builder.Append(selected.instanceId);
            builder.AppendLine(")");
        }

        private static void AppendRegistration(StringBuilder builder, UnityAiBridgeSelectionContextPayload payload)
        {
            builder.AppendLine();
            builder.Append("Registration status: ");
            builder.AppendLine(string.IsNullOrEmpty(payload.registrationStatus) ? "unknown" : payload.registrationStatus);
            if (!string.IsNullOrEmpty(payload.registrationUnavailableReason))
            {
                builder.Append("Unavailable reason: ");
                builder.AppendLine(payload.registrationUnavailableReason);
            }

            if (!string.IsNullOrEmpty(payload.contextPath))
            {
                builder.Append("Context path: ");
                builder.AppendLine(payload.contextPath);
            }

            if (!string.IsNullOrEmpty(payload.contextEventsPath))
            {
                builder.Append("Context events path: ");
                builder.AppendLine(payload.contextEventsPath);
            }
        }
    }
}

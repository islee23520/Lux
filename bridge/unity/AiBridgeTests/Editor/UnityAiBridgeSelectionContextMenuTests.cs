using System.IO;
using NUnit.Framework;
using UnityEditor;
using UnityEngine;
using UnityEngine.TestTools;

namespace Linalab.UnityAiBridge.Editor.Tests
{
    public sealed class UnityAiBridgeSelectionContextMenuTests
    {
        [SetUp]
        public void SetUp()
        {
            UnityAiBridgeSelectionContextEvidence.ProjectRootOverrideForTests = null;
        }

        [TearDown]
        public void TearDown()
        {
            UnityAiBridgeSelectionContextEvidence.ProjectRootOverrideForTests = null;
        }

        [Test]
        public void CopySelectionContext_WritesSummaryAndJsonToClipboard()
        {
            var previousSelection = Selection.objects;
            var previousClipboard = EditorGUIUtility.systemCopyBuffer;
            var projectRoot = CreateTempProjectRoot();
            var root = new GameObject("LuxContextRoot");
            var child = new GameObject("LuxContextChild");

            try
            {
                UnityAiBridgeSelectionContextEvidence.ProjectRootOverrideForTests = projectRoot;
                child.transform.SetParent(root.transform, false);
                Selection.objects = new Object[] { root };

                UnityAiBridgeAstContextMenu.CopySelectionContext();

                var clipboard = EditorGUIUtility.systemCopyBuffer;
                Assert.That(clipboard, Does.StartWith("Lux Unity selection context: /LuxContextRoot."));
                Assert.That(clipboard, Does.Contain("```json"));
                Assert.That(clipboard, Does.Contain("\"contextKind\": \"unity_selection_context\""));
                Assert.That(clipboard, Does.Contain("\"selectionCount\": 1"));
                Assert.That(clipboard, Does.Contain("\"hierarchyPath\": \"/LuxContextRoot\""));
                Assert.That(clipboard, Does.Contain("\"name\": \"LuxContextChild\""));
                Assert.That(clipboard, Does.Contain("\"globalObjectId\""));
                Assert.That(clipboard, Does.Contain("\"assetPath\""));
                Assert.That(clipboard, Does.Contain("\"assetGuid\""));
                Assert.That(clipboard, Does.Contain("\"prefabAssetPath\""));
                Assert.That(clipboard, Does.Contain("\"prefabAssetGuid\""));
                Assert.That(clipboard, Does.Contain("\"localFileId\""));
                Assert.That(clipboard, Does.Contain("\"instanceId\""));
                Assert.That(clipboard, Does.Contain("Registration status: registered"));
                Assert.That(clipboard, Does.Contain("\"registrationStatus\": \"registered\""));
                Assert.That(clipboard, Does.Contain("\"contextPath\""));
                Assert.That(clipboard, Does.Contain("\"contextEventsPath\""));

                var contextPath = Path.Combine(projectRoot, ".lux", "context", "selection-context.json");
                var eventsPath = Path.Combine(projectRoot, ".lux", "context", "selection-context-events.jsonl");
                Assert.That(File.Exists(contextPath), Is.True);
                Assert.That(File.Exists(eventsPath), Is.True);
                Assert.That(File.ReadAllText(contextPath), Does.Contain("\"registrationStatus\": \"registered\""));
                Assert.That(File.ReadAllText(eventsPath), Does.Contain("\"eventType\":\"unity.selection_context_copied\""));
            }
            finally
            {
                Selection.objects = previousSelection;
                EditorGUIUtility.systemCopyBuffer = previousClipboard;
                Object.DestroyImmediate(root);
                DeleteDirectoryIfExists(projectRoot);
            }
        }

        [Test]
        public void BuildSelectionContextEvidencePaths_DeriveFromApplicationDataPathProjectRoot()
        {
            var projectRoot = CreateTempProjectRoot();
            var assetsPath = Path.Combine(projectRoot, "Assets");

            try
            {
                Directory.CreateDirectory(assetsPath);

                string resolvedProjectRoot;
                string unavailableReason;
                var resolved = UnityAiBridgeSelectionContextEvidence.TryBuildUnityProjectRootFromDataPath(assetsPath, out resolvedProjectRoot, out unavailableReason);

                Assert.That(resolved, Is.True);
                Assert.That(unavailableReason, Is.Empty);
                Assert.That(resolvedProjectRoot, Is.EqualTo(projectRoot));

                string contextPath;
                string contextEventsPath;
                resolved = UnityAiBridgeSelectionContextEvidence.TryBuildPaths(resolvedProjectRoot, out contextPath, out contextEventsPath, out unavailableReason);

                Assert.That(resolved, Is.True);
                Assert.That(unavailableReason, Is.Empty);
                Assert.That(contextPath, Is.EqualTo(Path.Combine(projectRoot, ".lux", "context", "selection-context.json")));
                Assert.That(contextEventsPath, Is.EqualTo(Path.Combine(projectRoot, ".lux", "context", "selection-context-events.jsonl")));
            }
            finally
            {
                DeleteDirectoryIfExists(projectRoot);
            }
        }

        [Test]
        public void CopySelectionContext_UnavailableProjectRootWritesWarningAndClipboardReason()
        {
            var previousSelection = Selection.objects;
            var previousClipboard = EditorGUIUtility.systemCopyBuffer;

            try
            {
                UnityAiBridgeSelectionContextEvidence.ProjectRootOverrideForTests = string.Empty;
                Selection.objects = new Object[0];
                LogAssert.Expect(LogType.Warning, "Lux selection context evidence unavailable: Unity project root was unavailable; selection context evidence was not written.");

                UnityAiBridgeAstContextMenu.CopySelectionContext();

                var clipboard = EditorGUIUtility.systemCopyBuffer;
                Assert.That(clipboard, Does.Contain("\"registrationStatus\": \"unavailable\""));
                Assert.That(clipboard, Does.Contain("\"registrationUnavailableReason\": \"Unity project root was unavailable; selection context evidence was not written.\""));
            }
            finally
            {
                Selection.objects = previousSelection;
                EditorGUIUtility.systemCopyBuffer = previousClipboard;
            }
        }

        [Test]
        public void BuildSelectionContextClipboardText_NoSelectionWritesExplicitEmptyResult()
        {
            var previousSelection = Selection.objects;

            try
            {
                Selection.objects = new Object[0];

                var clipboard = UnityAiBridgeAstContextMenu.BuildSelectionContextClipboardText();

                Assert.That(clipboard, Does.StartWith("Lux Unity selection context: no GameObject is selected."));
                Assert.That(clipboard, Does.Contain("- No GameObject selected."));
                Assert.That(clipboard, Does.Contain("\"selectionCount\": 0"));
                Assert.That(clipboard, Does.Contain("\"selectedObjects\": []"));
                Assert.That(clipboard, Does.Contain("\"registrationStatus\": \"unavailable\""));
                Assert.That(clipboard, Does.Contain("\"registrationUnavailableReason\""));
            }
            finally
            {
                Selection.objects = previousSelection;
            }
        }

        [Test]
        public void CopySelectionContext_FromPropertyHighlightsSerializedPropertyPath()
        {
            var previousSelection = Selection.objects;
            var previousClipboard = EditorGUIUtility.systemCopyBuffer;
            var projectRoot = CreateTempProjectRoot();
            var gameObject = new GameObject("LuxPropertySelectionTarget");
            var serializedObject = new SerializedObject(gameObject.transform);
            var property = serializedObject.FindProperty("m_LocalScale");

            try
            {
                UnityAiBridgeSelectionContextEvidence.ProjectRootOverrideForTests = projectRoot;
                Selection.objects = new Object[] { gameObject };

                UnityAiBridgeAstContextMenu.CopySelectionContext(property);

                var clipboard = EditorGUIUtility.systemCopyBuffer;
                Assert.That(clipboard, Does.StartWith("Lux Unity selection context: property m_LocalScale on LuxPropertySelectionTarget."));
                Assert.That(clipboard, Does.Contain("- Highlighted property: m_LocalScale on LuxPropertySelectionTarget"));
                Assert.That(clipboard, Does.Contain("\"highlightedProperty\""));
                Assert.That(clipboard, Does.Contain("\"propertyPath\": \"m_LocalScale\""));
                Assert.That(clipboard, Does.Contain("\"displayName\": \"Local Scale\""));
                Assert.That(clipboard, Does.Contain("\"componentType\": \"UnityEngine.Transform\""));
                Assert.That(clipboard, Does.Contain("\"hierarchyPath\": \"/LuxPropertySelectionTarget\""));
                Assert.That(clipboard, Does.Contain("\"targetInstanceId\""));
                Assert.That(clipboard, Does.Contain("\"targetIdentifiers\""));
                Assert.That(clipboard, Does.Contain("\"registrationStatus\": \"registered\""));
            }
            finally
            {
                Selection.objects = previousSelection;
                EditorGUIUtility.systemCopyBuffer = previousClipboard;
                serializedObject.Dispose();
                Object.DestroyImmediate(gameObject);
                DeleteDirectoryIfExists(projectRoot);
            }
        }

        private static string CreateTempProjectRoot()
        {
            var projectRoot = Path.Combine(Path.GetTempPath(), "LuxSelectionContextTests", System.Guid.NewGuid().ToString("N"));
            Directory.CreateDirectory(projectRoot);
            return projectRoot;
        }

        private static void DeleteDirectoryIfExists(string path)
        {
            if (!string.IsNullOrEmpty(path) && Directory.Exists(path))
            {
                Directory.Delete(path, true);
            }
        }
    }
}

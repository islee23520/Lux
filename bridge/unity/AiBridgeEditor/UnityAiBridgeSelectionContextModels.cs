using System;
using Linalab.UnityAiBridge.Editor.Ast;

namespace Linalab.UnityAiBridge.Editor
{
    [Serializable]
    public sealed class UnityAiBridgeSelectionContextPayload
    {
        public int schemaVersion;
        public string contextKind;
        public string capturedAtUtc;
        public string summary;
        public string registrationStatus;
        public string registrationUnavailableReason;
        public string contextPath;
        public string contextEventsPath;
        public int selectionCount;
        public UnityAiBridgeSelectedObjectContext[] selectedObjects;
        public UnityAiBridgeHighlightedPropertyContext highlightedProperty;
        public UnityAstSelectionAstPayload selectionAst;
    }

    [Serializable]
    internal sealed class UnityAiBridgeSelectionContextCopiedEvent
    {
        public int schemaVersion;
        public string eventType;
        public string capturedAtUtc;
        public string contextPath;
        public int selectionCount;
        public string summary;
    }

    [Serializable]
    public sealed class UnityAiBridgeSelectedObjectContext
    {
        public string name;
        public string sceneName;
        public string scenePath;
        public string hierarchyPath;
        public bool activeSelf;
        public bool isStatic;
        public int layer;
        public string tag;
        public int instanceId;
        public UnityAiBridgeObjectIdentifiers identifiers;
        public UnityAiBridgeTransformContext transform;
    }

    [Serializable]
    public sealed class UnityAiBridgeHighlightedPropertyContext
    {
        public string path;
        public string targetKind;
        public string targetType;
        public string targetName;
        public string scenePath;
        public string hierarchyPath;
        public string componentType;
        public string assetPath;
        public string assetGuid;
        public string prefabAssetPath;
        public string prefabAssetGuid;
        public string propertyPath;
        public string displayName;
        public string propertyType;
        public string value;
        public int targetCount;
        public int targetInstanceId;
        public UnityAiBridgeObjectIdentifiers targetIdentifiers;
    }

    [Serializable]
    public sealed class UnityAiBridgeObjectIdentifiers
    {
        public string globalObjectId;
        public string assetPath;
        public string assetGuid;
        public string prefabAssetPath;
        public string prefabAssetGuid;
        public string localFileId;
        public string localFileGuid;
        public string unavailableReason;
    }

    [Serializable]
    public sealed class UnityAiBridgeTransformContext
    {
        public UnityAiBridgeVector3Context position;
        public UnityAiBridgeVector3Context localPosition;
        public UnityAiBridgeVector3Context rotationEuler;
        public UnityAiBridgeVector3Context localRotationEuler;
        public UnityAiBridgeVector3Context localScale;
    }

    [Serializable]
    public sealed class UnityAiBridgeVector3Context
    {
        public float x;
        public float y;
        public float z;
    }
}

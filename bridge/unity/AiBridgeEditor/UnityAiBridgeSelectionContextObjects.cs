using Linalab.UnityAiBridge.Editor.Ast;
using System;
using System.Collections.Generic;
using UnityEditor;
using UnityEngine;

namespace Linalab.UnityAiBridge.Editor
{
    internal static class UnityAiBridgeSelectionContextObjects
    {
        internal static UnityAiBridgeSelectedObjectContext[] BuildSelectedObjectContexts(GameObject[] gameObjects)
        {
            if (gameObjects == null || gameObjects.Length == 0)
            {
                return new UnityAiBridgeSelectedObjectContext[0];
            }

            var contexts = new List<UnityAiBridgeSelectedObjectContext>();
            for (var i = 0; i < gameObjects.Length; i++)
            {
                var go = gameObjects[i];
                if (go != null)
                {
                    contexts.Add(BuildSelectedObjectContext(go));
                }
            }

            return contexts.ToArray();
        }

        internal static UnityAiBridgeHighlightedPropertyContext BuildHighlightedProperty(SerializedProperty property)
        {
            var propertyContext = UnityAiBridgePropertyContextPath.FromProperty(property);
            var target = property.serializedObject == null ? null : property.serializedObject.targetObject;

            return new UnityAiBridgeHighlightedPropertyContext
            {
                path = propertyContext.ToPath(),
                targetKind = propertyContext.targetKind,
                targetType = propertyContext.targetType,
                targetName = propertyContext.targetName,
                scenePath = propertyContext.scenePath,
                hierarchyPath = propertyContext.hierarchyPath,
                componentType = propertyContext.componentType,
                assetPath = propertyContext.assetPath,
                assetGuid = propertyContext.assetGuid,
                prefabAssetPath = propertyContext.prefabAssetPath,
                prefabAssetGuid = propertyContext.prefabAssetGuid,
                propertyPath = propertyContext.propertyPath,
                displayName = propertyContext.displayName,
                propertyType = propertyContext.propertyType,
                value = FormatPropertyValue(property),
                targetCount = propertyContext.targetCount,
                targetInstanceId = target == null ? 0 : target.GetInstanceID(),
                targetIdentifiers = BuildIdentifiers(target)
            };
        }

        private static UnityAiBridgeSelectedObjectContext BuildSelectedObjectContext(GameObject gameObject)
        {
            var transform = gameObject.transform;
            var scene = gameObject.scene;

            return new UnityAiBridgeSelectedObjectContext
            {
                name = gameObject.name,
                sceneName = scene.IsValid() ? scene.name : string.Empty,
                scenePath = scene.IsValid() ? scene.path : string.Empty,
                hierarchyPath = BuildHierarchyPath(transform),
                activeSelf = gameObject.activeSelf,
                isStatic = gameObject.isStatic,
                layer = gameObject.layer,
                tag = UnityAstConstants.GetTag(gameObject),
                instanceId = gameObject.GetInstanceID(),
                identifiers = BuildIdentifiers(gameObject),
                transform = BuildTransformContext(transform)
            };
        }

        private static UnityAiBridgeTransformContext BuildTransformContext(Transform transform)
        {
            if (transform == null)
            {
                return null;
            }

            return new UnityAiBridgeTransformContext
            {
                position = ToVector3Context(transform.position),
                localPosition = ToVector3Context(transform.localPosition),
                rotationEuler = ToVector3Context(transform.rotation.eulerAngles),
                localRotationEuler = ToVector3Context(transform.localRotation.eulerAngles),
                localScale = ToVector3Context(transform.localScale)
            };
        }

        private static UnityAiBridgeVector3Context ToVector3Context(Vector3 value)
        {
            return new UnityAiBridgeVector3Context
            {
                x = value.x,
                y = value.y,
                z = value.z
            };
        }

        private static UnityAiBridgeObjectIdentifiers BuildIdentifiers(UnityEngine.Object target)
        {
            var identifiers = new UnityAiBridgeObjectIdentifiers
            {
                globalObjectId = string.Empty,
                assetPath = string.Empty,
                assetGuid = string.Empty,
                prefabAssetPath = string.Empty,
                prefabAssetGuid = string.Empty,
                localFileId = string.Empty,
                localFileGuid = string.Empty,
                unavailableReason = string.Empty
            };

            if (target == null)
            {
                identifiers.unavailableReason = "No Unity object target was available.";
                return identifiers;
            }

            identifiers.globalObjectId = GlobalObjectId.GetGlobalObjectIdSlow(target).ToString();
            identifiers.assetPath = AssetDatabase.GetAssetPath(target) ?? string.Empty;
            identifiers.assetGuid = string.IsNullOrEmpty(identifiers.assetPath) ? string.Empty : AssetDatabase.AssetPathToGUID(identifiers.assetPath);

            var gameObject = target as GameObject;
            var component = target as Component;
            if (component != null)
            {
                gameObject = component.gameObject;
            }

            if (gameObject != null)
            {
                identifiers.prefabAssetPath = PrefabUtility.GetPrefabAssetPathOfNearestInstanceRoot(gameObject) ?? string.Empty;
                identifiers.prefabAssetGuid = string.IsNullOrEmpty(identifiers.prefabAssetPath) ? string.Empty : AssetDatabase.AssetPathToGUID(identifiers.prefabAssetPath);
            }
            else if (!string.IsNullOrEmpty(identifiers.assetPath) && identifiers.assetPath.EndsWith(".prefab", StringComparison.OrdinalIgnoreCase))
            {
                identifiers.prefabAssetPath = identifiers.assetPath;
                identifiers.prefabAssetGuid = identifiers.assetGuid;
            }

            string guid;
            long localFileId;
            if (AssetDatabase.TryGetGUIDAndLocalFileIdentifier(target, out guid, out localFileId))
            {
                identifiers.localFileGuid = guid ?? string.Empty;
                identifiers.localFileId = localFileId.ToString();
            }
            else
            {
                identifiers.unavailableReason = "AssetDatabase could not extract a local file identifier for this object.";
            }

            return identifiers;
        }

        private static string BuildHierarchyPath(Transform transform)
        {
            if (transform == null)
            {
                return string.Empty;
            }

            var path = transform.name;
            var current = transform.parent;
            while (current != null)
            {
                path = current.name + "/" + path;
                current = current.parent;
            }

            return "/" + path;
        }

        private static string FormatPropertyValue(SerializedProperty property)
        {
            switch (property.propertyType)
            {
                case SerializedPropertyType.Integer:
                    return property.intValue.ToString();
                case SerializedPropertyType.Boolean:
                    return property.boolValue ? "true" : "false";
                case SerializedPropertyType.Float:
                    return property.floatValue.ToString("R");
                case SerializedPropertyType.String:
                    return property.stringValue ?? string.Empty;
                case SerializedPropertyType.Color:
                    return property.colorValue.ToString();
                case SerializedPropertyType.ObjectReference:
                    return property.objectReferenceValue == null ? string.Empty : property.objectReferenceValue.name;
                case SerializedPropertyType.Enum:
                    return property.enumDisplayNames != null && property.enumValueIndex >= 0 && property.enumValueIndex < property.enumDisplayNames.Length
                        ? property.enumDisplayNames[property.enumValueIndex]
                        : property.enumValueIndex.ToString();
                case SerializedPropertyType.Vector2:
                    return property.vector2Value.ToString();
                case SerializedPropertyType.Vector3:
                    return property.vector3Value.ToString();
                case SerializedPropertyType.Vector4:
                    return property.vector4Value.ToString();
                case SerializedPropertyType.Quaternion:
                    return property.quaternionValue.ToString();
                case SerializedPropertyType.Rect:
                    return property.rectValue.ToString();
                case SerializedPropertyType.Bounds:
                    return property.boundsValue.ToString();
                default:
                    return property.propertyType.ToString();
            }
        }
    }
}

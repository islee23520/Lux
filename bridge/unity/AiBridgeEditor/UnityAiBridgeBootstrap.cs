using UnityEditor;
using UnityEngine;

namespace Linalab.UnityAiBridge.Editor
{
    [InitializeOnLoad]
    internal static class UnityAiBridgeBootstrap
    {
        private static readonly bool EnableStartupContextServer = true;
        private const double EnsureIntervalSeconds = 5.0d;
        private static double nextEnsureTime;

        static UnityAiBridgeBootstrap()
        {
            // Write bridge settings on every import (safe in both interactive and batchmode)
            try
            {
                Linalab.Lux.Editor.LuxBridgeSettings.WriteProjectSettings();
            }
            catch (System.Exception e)
            {
                UnityEngine.Debug.LogWarning($"Failed to auto-write LuxBridgeSettings: {e.Message}");
            }

            EditorApplication.delayCall += StartIfEnabled;
            EditorApplication.update += EnsureRunningIfEnabled;
        }

        private static void StartIfEnabled()
        {
            if (!EnableStartupContextServer)
            {
                return;
            }

            if (Application.isBatchMode)
            {
                return;
            }

            if (!UnityAiBridgeMenu.GetAutoStartEnabled())
            {
                return;
            }

            UnityAiBridgeTcpServer.EnsureSharedDiscoverable();
        }

        private static void EnsureRunningIfEnabled()
        {
            if (EditorApplication.timeSinceStartup < nextEnsureTime)
            {
                return;
            }

            nextEnsureTime = EditorApplication.timeSinceStartup + EnsureIntervalSeconds;

            if (!EnableStartupContextServer || Application.isBatchMode || !UnityAiBridgeMenu.GetAutoStartEnabled())
            {
                return;
            }

            try
            {
                UnityAiBridgeTcpServer.EnsureSharedDiscoverable();
            }
            catch (System.Exception exception)
            {
                Debug.LogWarning($"Lux Unity AI Bridge backend auto-start failed: {exception.Message}");
            }
        }
    }
}

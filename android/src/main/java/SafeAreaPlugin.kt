package com.plugin.edge

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class EnableArgs {
    var config: JSObject? = null
}

@InvokeArg
class DisableArgs {
    var config: JSObject? = null
}

@TauriPlugin
class SafeAreaPlugin(private val activity: Activity) : Plugin(activity) {
    private var implementation: SafeArea? = null

    override fun onLoad() {
        implementation = SafeArea(activity, bridge.webView)

        val enabled = bridge.config.getBoolean("enabled", false)

        if (enabled) {
            implementation?.offset = bridge.config.getInt("offset", 0)
            implementation?.enable(false, AppearanceConfig(bridge.config))
        }
    }

    override fun onPause() {
        implementation?.resetDecorFitsSystemWindows()
        super.onPause()
    }

    @Command
    fun enable(invoke: Invoke) {
        val args = invoke.parseArgs(EnableArgs::class.java)
        val config = args.config ?: JSObject()

        if (config.has("offset")) {
            implementation?.offset = config.getInt("offset")
        }

        val appearanceConfig = AppearanceConfig(config)
        implementation?.enable(true, appearanceConfig)

        invoke.resolve()
    }

    @Command
    fun disable(invoke: Invoke) {
        val args = invoke.parseArgs(DisableArgs::class.java)
        val config = args.config ?: JSObject()

        val appearanceConfig = AppearanceConfig(config)
        implementation?.disable(appearanceConfig)

        invoke.resolve()
    }
}

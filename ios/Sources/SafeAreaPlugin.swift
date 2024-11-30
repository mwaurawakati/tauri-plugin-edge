import Tauri
import UIKit
import WebKit

// Struct to decode the arguments for the `enable` command
struct EnableArgs: Decodable {
    let config: [String: String]?

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        config = try container.decodeIfPresent([String: String].self, forKey: .config)
    }

    private enum CodingKeys: String, CodingKey {
        case config
    }
}

struct DisableArgs: Decodable {
    let config: [String: String]?

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        config = try container.decodeIfPresent([String: String].self, forKey: .config)
    }

    private enum CodingKeys: String, CodingKey {
        case config
    }
}


// SafeArea implementation stub
class SafeArea {
    var offset: Int = 0
    private weak var webView: WKWebView?

    init(webView: WKWebView?) {
        self.webView = webView
    }

    func enable(_ animated: Bool, appearanceConfig: AppearanceConfig) {
        // Enable logic here
    }

    func disable(_ appearanceConfig: AppearanceConfig) {
        // Disable logic here
    }

    func resetDecorFitsSystemWindows() {
        // Reset system windows logic here
    }
}

// AppearanceConfig struct equivalent
struct AppearanceConfig {
    var customColorsForSystemBars: Bool = true
    var statusBarColor: String = "#000000"
    var statusBarContent: String = "light"
    var navigationBarColor: String = "#000000"
    var navigationBarContent: String = "light"

    init(config: [String: Any]?) {
        if let config = config {
            self.customColorsForSystemBars = config["customColorsForSystemBars"] as? Bool ?? true
            self.statusBarColor = config["statusBarColor"] as? String ?? "#000000"
            self.statusBarContent = config["statusBarContent"] as? String ?? "light"
            self.navigationBarColor = config["navigationBarColor"] as? String ?? "#000000"
            self.navigationBarContent = config["navigationBarContent"] as? String ?? "light"
        }
    }
}

// SafeAreaPlugin implementation
class SafeAreaPlugin: Plugin {
    private var safeArea: SafeArea?
    private var isEnabled: Bool = false
    private var webView: WKWebView?
    private var bridge: SomeBridgeType?
    // Plugin initialization
    func load() {
        guard let webView = self.webView else { return }
        self.safeArea = SafeArea(webView: webView)

        let enabled = self.bridge.config["enabled"] as? Bool ?? false
        if enabled {
            let offset = self.bridge.config["offset"] as? Int ?? 0
            self.safeArea?.offset = offset
            let appearanceConfig = AppearanceConfig(config: self.bridge.config)
            self.safeArea?.enable(false, appearanceConfig: appearanceConfig)
        }
    }

    func pause() {
        self.safeArea?.resetDecorFitsSystemWindows()
        super.pause()
    }

    @objc public func enable(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(EnableArgs.self)
        let config = args.config ?? [:]

        if let offset = config["offset"] as? Int {
            self.safeArea?.offset = offset
        }

        let appearanceConfig = AppearanceConfig(config: config)
        self.safeArea?.enable(true, appearanceConfig: appearanceConfig)
        invoke.resolve()
    }

    @objc public func disable(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(DisableArgs.self)
        let config = args.config ?? [:]

        let appearanceConfig = AppearanceConfig(config: config)
        self.safeArea?.disable(appearanceConfig: appearanceConfig)
        invoke.resolve()
    }
}

@_cdecl("init_plugin_edge")
func initPlugin() -> Plugin {
    return SafeAreaPlugin()
}

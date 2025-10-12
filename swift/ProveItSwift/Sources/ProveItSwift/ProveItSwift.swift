//
//  ProveItSwift.swift
//  ProveIt SwiftUI Bindings
//
//  Swift/SwiftUI interface for the ProveIt Rust core.
//  Provides native macOS and iOS support for geometric proof construction.
//

import Foundation
import SwiftUI

/// Position in 3D space
public struct Position: Codable, Equatable {
    public let x: Double
    public let y: Double
    public let z: Double
    
    public init(x: Double, y: Double, z: Double) {
        self.x = x
        self.y = y
        self.z = z
    }
    
    public static var origin: Position {
        Position(x: 0, y: 0, z: 0)
    }
}

/// Proof object identifier
public struct ProofID: Codable, Equatable, Hashable {
    public let id: UInt64
    
    public init(_ id: UInt64) {
        self.id = id
    }
}

/// Accessibility settings for ProveIt
public struct AccessibilitySettings {
    public var screenReaderEnabled: Bool
    public var spatialAudioEnabled: Bool
    public var highContrast: Bool
    public var reduceMotion: Bool
    public var fontSizeMultiplier: Double
    
    public init(
        screenReaderEnabled: Bool = false,
        spatialAudioEnabled: Bool = false,
        highContrast: Bool = false,
        reduceMotion: Bool = false,
        fontSizeMultiplier: Double = 1.0
    ) {
        self.screenReaderEnabled = screenReaderEnabled
        self.spatialAudioEnabled = spatialAudioEnabled
        self.highContrast = highContrast
        self.reduceMotion = reduceMotion
        self.fontSizeMultiplier = fontSizeMultiplier
    }
    
    /// Preset for blind users
    public static var blindPreset: AccessibilitySettings {
        AccessibilitySettings(
            screenReaderEnabled: true,
            spatialAudioEnabled: true,
            highContrast: false,
            reduceMotion: false,
            fontSizeMultiplier: 1.0
        )
    }
    
    /// Preset for visually impaired users
    public static var visuallyImpairedPreset: AccessibilitySettings {
        AccessibilitySettings(
            screenReaderEnabled: true,
            spatialAudioEnabled: false,
            highContrast: true,
            reduceMotion: false,
            fontSizeMultiplier: 2.0
        )
    }
    
    /// Preset for neurodivergent users
    public static var neurodivergentPreset: AccessibilitySettings {
        AccessibilitySettings(
            screenReaderEnabled: false,
            spatialAudioEnabled: false,
            highContrast: false,
            reduceMotion: true,
            fontSizeMultiplier: 1.2
        )
    }
}

/// ProveIt core interface
@available(macOS 13.0, iOS 16.0, *)
public class ProveItCore: ObservableObject {
    @Published public var accessibilitySettings: AccessibilitySettings
    
    public init() {
        self.accessibilitySettings = AccessibilitySettings()
    }
    
    /// Initialize with specific accessibility settings
    public init(accessibilitySettings: AccessibilitySettings) {
        self.accessibilitySettings = accessibilitySettings
    }
    
    /// Verify a proof (placeholder - will call Rust FFI)
    public func verify() -> Bool {
        // TODO: Call Rust core via FFI
        return true
    }
    
    /// Get description for screen readers
    public func describe() -> String {
        return "ProveIt geometric verification system"
    }
}

/// SwiftUI view for ProveIt interface
@available(macOS 13.0, iOS 16.0, *)
public struct ProveItView: View {
    @ObservedObject var core: ProveItCore
    
    public init(core: ProveItCore) {
        self.core = core
    }
    
    public var body: some View {
        VStack {
            Text("ProveIt")
                .font(.system(size: 32 * core.accessibilitySettings.fontSizeMultiplier))
                .padding()
            
            Text("Geometric Proof Construction")
                .font(.system(size: 16 * core.accessibilitySettings.fontSizeMultiplier))
                .foregroundColor(core.accessibilitySettings.highContrast ? .primary : .secondary)
            
            Spacer()
            
            // Accessibility settings panel
            if core.accessibilitySettings.screenReaderEnabled {
                Text("Screen reader mode active")
                    .accessibilityLabel("Screen reader mode is currently active")
            }
        }
        .padding()
        .preferredColorScheme(core.accessibilitySettings.highContrast ? .dark : .light)
        .accessibilityElement(children: .contain)
    }
}

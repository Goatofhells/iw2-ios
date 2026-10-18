import SwiftUI
import UniformTypeIdentifiers

@main
struct pocket: App {
    var body: some Scene {
        WindowGroup { home().preferredColorScheme(.dark) }
    }
}

struct home: View {
    @State private var picker = false
    @State private var busy = false
    @State private var status = "No verified installation"
    @State private var cmd = "config"
    @State private var job = UUID()
    @State private var src: URL?
    @State private var count = 0
    @AppStorage("fps") private var fps = 30
    @AppStorage("scale") private var scale = 0.75
    @AppStorage("cache") private var cache = 128
    @AppStorage("sense") private var sense = 1.0
    @AppStorage("invert") private var invert = false

    var body: some View {
        NavigationView {
            List {
                Section {
                    VStack(alignment: .leading, spacing: 12) {
                        Text("IW4 POCKET").font(.largeTitle.bold())
                        Text("Your field kit. Built for iPhone.").foregroundColor(.secondary)
                        Label("Launcher foundation", systemImage: "checkmark.circle.fill").foregroundColor(.mint)
                        Text("MW2 runtime unavailable").font(.caption.bold()).foregroundColor(.orange)
                    }.padding(.vertical, 18)
                }
                Section("Installation") {
                    Button { picker = true } label: {
                        Label("Import Game Files", systemImage: "square.and.arrow.down")
                    }.disabled(busy)
                    Button {
                        cmd = "verify"
                        job = UUID()
                    } label: {
                        Label("Verify Installation", systemImage: "checkmark.shield")
                    }.disabled(busy)
                    Text(status).font(.footnote).foregroundColor(.secondary)
                    if busy { ProgressView("Working…") }
                }
                Section("Play") {
                    NavigationLink("Offline Practice") {
                        VStack(spacing: 18) {
                            Image(systemName: "cube.transparent").font(.largeTitle)
                            Text("Practice arena is not implemented").font(.title2.bold())
                            Text("The next milestone is an original 3D arena with touch controls. No MW2 engine or assets are included.")
                        }.padding().navigationTitle("Offline Practice")
                    }
                    NavigationLink("Multiplayer") {
                        VStack(spacing: 18) {
                            Image(systemName: "network").font(.largeTitle)
                            Text("Multiplayer is not implemented").font(.title2.bold())
                            Text("No server connections or MW2 protocol compatibility are available.")
                        }.padding().navigationTitle("Multiplayer")
                    }
                }
                Section("Preferences") {
                    NavigationLink("Settings") {
                        Form {
                            Section("Future renderer preferences") {
                                Picker("Frame rate target", selection: $fps) {
                                    Text("30 FPS").tag(30)
                                    Text("60 FPS").tag(60)
                                }
                                Slider(value: $scale, in: 0.5...1.0, step: 0.05) {
                                    Text("Resolution scale")
                                }
                                Text("Resolution: \(Int(scale * 100))%")
                                Stepper("Cache cap: \(cache) MiB", value: $cache, in: 16...512, step: 16)
                                Text("Saved preferences. There is no renderer yet; frame rates have not been benchmarked.").font(.footnote)
                            }
                            Section("Future touch controls") {
                                Slider(value: $sense, in: 0.25...3, step: 0.25) { Text("Aim sensitivity") }
                                Text("Sensitivity: \(sense, specifier: "%.2f")")
                                Toggle("Invert vertical aim", isOn: $invert)
                                Text("Movement, aiming, fire, aim, reload, jump, crouch, sprint, pause, and controller support await the prototype.").font(.footnote)
                            }
                        }.navigationTitle("Settings")
                    }
                    NavigationLink("Diagnostics") {
                        List {
                            row(title: "System", value: UIDevice.current.systemVersion)
                            row(title: "Device", value: UIDevice.current.model)
                            row(title: "Physical memory", value: "\(ProcessInfo.processInfo.physicalMemory / 1048576) MiB")
                            row(title: "Imported sets", value: "\(count)")
                            Text("Rust services linked through C ABI. Imports are unverified until a trusted manifest is available. Hash checks do not prove ownership.")
                            Button("Check memory settings") {
                                cmd = "config"
                                job = UUID()
                            }.disabled(busy)
                            Text(status).font(.footnote)
                        }.navigationTitle("Diagnostics")
                    }
                }
            }
            .listStyle(.insetGrouped)
            .navigationTitle("Home")
            .tint(.mint)
            .fileImporter(isPresented: $picker, allowedContentTypes: [.zip, .data], allowsMultipleSelection: false) { res in
                switch res {
                case .success(let urls):
                    src = urls.first
                    cmd = "import"
                    job = UUID()
                case .failure(let err): status = err.localizedDescription
                }
            }
            .task(id: job) {
                busy = true
                let url = src
                let op = cmd
                let cfg: [String: Any] = ["fps": fps, "scale": scale, "cache": cache]
                let ram = ProcessInfo.processInfo.physicalMemory / 1048576
                let root = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0].appendingPathComponent("Imports", isDirectory: true)
                let base: [String: Any] = ["op": op, "ram": ram, "config": cfg]
                let data = (try? JSONSerialization.data(withJSONObject: base)) ?? Data()
                let res = await Task.detached(priority: .userInitiated) { () -> (String, Int) in
                    let fm = FileManager.default
                    do {
                        try fm.createDirectory(at: root, withIntermediateDirectories: true)
                        var req = try JSONSerialization.jsonObject(with: data) as! [String: Any]
                        if op == "import" {
                            guard let url = url else { throw CocoaError(.fileNoSuchFile) }
                            let access = url.startAccessingSecurityScopedResource()
                            defer { if access { url.stopAccessingSecurityScopedResource() } }
                            let ext = url.pathExtension.lowercased()
                            guard ["zip", "ff", "iwd", "cfg"].contains(ext) else {
                                return ("Unsupported file. Select a ZIP, FF, IWD, or CFG file.", (try fm.contentsOfDirectory(atPath: root.path)).count)
                            }
                            let dst = root.appendingPathComponent(UUID().uuidString, isDirectory: true)
                            if ext != "zip" {
                                try fm.createDirectory(at: dst, withIntermediateDirectories: true)
                                do { try fm.copyItem(at: url, to: dst.appendingPathComponent(url.lastPathComponent)) }
                                catch { try? fm.removeItem(at: dst); throw error }
                                return ("File imported into sandbox. Installation unsupported; no approved MW2 manifest is shipped.", (try fm.contentsOfDirectory(atPath: root.path)).count)
                            }
                            req["src"] = url.path
                            req["dst"] = dst.path
                            let bytes = try JSONSerialization.data(withJSONObject: req)
                            let text = String(decoding: bytes, as: UTF8.self)
                            guard let ptr = text.withCString({ iw4_run($0) }) else { throw CocoaError(.coderInvalidValue) }
                            defer { iw4_free(ptr) }
                            let out = Data(String(cString: ptr).utf8)
                            let doc = try JSONSerialization.jsonObject(with: out) as! [String: Any]
                            return (doc["message"] as? String ?? "Service response invalid", (try fm.contentsOfDirectory(atPath: root.path)).count)
                        }
                        req["src"] = root.path
                        let bytes = try JSONSerialization.data(withJSONObject: req)
                        let text = String(decoding: bytes, as: UTF8.self)
                        guard let ptr = text.withCString({ iw4_run($0) }) else { throw CocoaError(.coderInvalidValue) }
                        defer { iw4_free(ptr) }
                        let out = Data(String(cString: ptr).utf8)
                        let doc = try JSONSerialization.jsonObject(with: out) as! [String: Any]
                        return (doc["message"] as? String ?? "Service response invalid", (try fm.contentsOfDirectory(atPath: root.path)).count)
                    } catch { return (error.localizedDescription, 0) }
                }.value
                status = res.0
                count = res.1
                busy = false
            }
        }.navigationViewStyle(.stack)
    }
}

struct row: View {
    let title: String
    let value: String
    var body: some View {
        HStack { Text(title); Spacer(); Text(value).foregroundColor(.secondary) }
    }
}

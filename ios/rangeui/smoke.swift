import XCTest

final class smoke: XCTestCase {
    func test_range() {
        let app = XCUIApplication(bundleIdentifier: "com.iw4pocket.range")
        app.launch()
        XCTAssertTrue(app.wait(for: .runningForeground, timeout: 20))
        let win = app.windows.firstMatch
        XCTAssertTrue(win.waitForExistence(timeout: 10))
        let menu = XCTAttachment(screenshot: app.screenshot())
        menu.name = "Rust range menu"
        menu.lifetime = .keepAlways
        add(menu)
        win.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.64)).tap()
        win.coordinate(withNormalizedOffset: CGVector(dx: 0.90, dy: 0.81)).tap()
        win.coordinate(withNormalizedOffset: CGVector(dx: 0.90, dy: 0.81)).tap()
        let arena = XCTAttachment(screenshot: app.screenshot())
        arena.name = "Rust range after firing"
        arena.lifetime = .keepAlways
        add(arena)
        XCTAssertEqual(app.state, .runningForeground)
        win.coordinate(withNormalizedOffset: CGVector(dx: 0.95, dy: 0.09)).tap()
    }
}

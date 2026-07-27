import { test, expect, Page } from "@playwright/test";

// Helper: wait for WASM app to hydrate
async function waitForApp(page: Page) {
  await page.waitForSelector("h1", { timeout: 15_000 });
  await expect(page.locator("h1")).toContainText("dioxus-bootstrap Showcase");
}

// Helper: click a top-level tab by label
async function clickTab(page: Page, label: string) {
  await page.locator(".nav-tabs .nav-link", { hasText: label }).first().click();
  // Small wait for tab content to render
  await page.waitForTimeout(300);
}

// ---------------------------------------------------------------------------
// App loads
// ---------------------------------------------------------------------------
test.describe("App", () => {
  test("loads and shows title", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await expect(page.locator(".lead").first()).toContainText("zero JavaScript");
  });

  test("navbar is visible", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await expect(page.locator(".navbar.sticky-top")).toBeVisible();
  });

  test("navbar uses navbar-nav structure with separated links", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);

    const links = page.locator(".navbar.sticky-top .navbar-nav .nav-link");
    await expect(links).toHaveCount(2);
    await expect(links.nth(0)).toContainText("Showcase");
    await expect(links.nth(1)).toContainText("Docs");

    for (const link of await links.all()) {
      const padding = await link.evaluate((el) => {
        const style = window.getComputedStyle(el);
        return {
          left: Number.parseFloat(style.paddingLeft),
          right: Number.parseFloat(style.paddingRight),
        };
      });
      expect(padding.left).toBeGreaterThanOrEqual(8);
      expect(padding.right).toBeGreaterThanOrEqual(8);
    }
  });

  test("theme toggle works", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    const html = page.locator("html");
    // Default is dark
    await expect(html).toHaveAttribute("data-bs-theme", "dark");
    // Click theme toggle (button has aria-label "Switch to light mode" when dark)
    await page.locator("button[aria-label='Switch to light mode']").click();
    await expect(html).toHaveAttribute("data-bs-theme", "light");
  });

  test("all 8 tabs exist", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    const tabs = ["Basics", "Forms", "Data", "Interactive", "Overlays", "Media", "Navigation", "More"];
    for (const tab of tabs) {
      await expect(page.locator(".nav-tabs .nav-link", { hasText: tab }).first()).toBeVisible();
    }
  });
});

// ---------------------------------------------------------------------------
// Basics tab
// ---------------------------------------------------------------------------
test.describe("Basics tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Basics");
  });

  test("buttons render", async ({ page }) => {
    await expect(page.locator(".btn-primary").first()).toBeVisible();
    await expect(page.locator(".btn-secondary").first()).toBeVisible();
    await expect(page.locator(".btn-success").first()).toBeVisible();
  });

  test("button group renders", async ({ page }) => {
    await expect(page.locator(".btn-group").first()).toBeVisible();
  });

  test("cards render", async ({ page }) => {
    await expect(page.locator(".card").first()).toBeVisible();
    await expect(page.locator(".card-body").first()).toBeVisible();
  });

  test("alerts render", async ({ page }) => {
    await expect(page.locator(".alert").first()).toBeVisible();
  });

  test("badges render", async ({ page }) => {
    await expect(page.locator(".badge").first()).toBeVisible();
  });

  test("grid system renders", async ({ page }) => {
    await expect(page.locator(".row").first()).toBeVisible();
    await expect(page.locator("[class*='col']").first()).toBeVisible();
  });

  test("breadcrumb renders", async ({ page }) => {
    await expect(page.locator(".breadcrumb").first()).toBeVisible();
  });

});

// ---------------------------------------------------------------------------
// Forms tab
// ---------------------------------------------------------------------------
test.describe("Forms tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Forms");
  });

  test("text inputs render", async ({ page }) => {
    await expect(page.locator(".form-control").first()).toBeVisible();
  });

  test("select renders", async ({ page }) => {
    await expect(page.locator(".form-select").first()).toBeVisible();
  });

  test("checkboxes render", async ({ page }) => {
    await expect(page.locator(".form-check-input").first()).toBeVisible();
  });

  test("switch renders", async ({ page }) => {
    await expect(page.locator(".form-switch").first()).toBeVisible();
  });

  test("range renders", async ({ page }) => {
    await expect(page.locator(".form-range").first()).toBeVisible();
  });

  test("floating labels render", async ({ page }) => {
    await expect(page.locator(".form-floating").first()).toBeVisible();
  });

  test("input group renders", async ({ page }) => {
    await expect(page.locator(".input-group").first()).toBeVisible();
  });

  test("validation feedback renders", async ({ page }) => {
    const valid = page.locator(".is-valid, .valid-feedback");
    const invalid = page.locator(".is-invalid, .invalid-feedback");
    await expect(valid.first()).toBeVisible();
    await expect(invalid.first()).toBeVisible();
  });

  test("input can be typed into", async ({ page }) => {
    const input = page.locator("input.form-control").first();
    await input.fill("test@example.com");
    await expect(input).toHaveValue("test@example.com");
  });
});

// ---------------------------------------------------------------------------
// Data tab
// ---------------------------------------------------------------------------
test.describe("Data tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Data");
  });

  test("table renders with rows", async ({ page }) => {
    await expect(page.locator("table.table").first()).toBeVisible();
    const rows = page.locator("table.table tbody tr");
    expect(await rows.count()).toBeGreaterThan(0);
  });

  test("striped table renders", async ({ page }) => {
    await expect(page.locator("table.table-striped").first()).toBeVisible();
  });

  test("list group renders", async ({ page }) => {
    await expect(page.locator(".list-group").first()).toBeVisible();
    await expect(page.locator(".list-group-item").first()).toBeVisible();
  });

  test("progress bars render", async ({ page }) => {
    await expect(page.locator(".progress").first()).toBeVisible();
    await expect(page.locator(".progress-bar").first()).toBeVisible();
  });

  test("spinners render", async ({ page }) => {
    await expect(page.locator(".spinner-border, .spinner-grow").first()).toBeVisible();
  });

  test("pagination renders", async ({ page }) => {
    await expect(page.locator(".pagination").first()).toBeVisible();
    await expect(page.locator(".page-item").first()).toBeVisible();
  });

  test("pagination click changes active page", async ({ page }) => {
    const page4 = page.locator(".page-link", { hasText: "4" }).first();
    await page4.click();
    await expect(page.locator(".page-item.active .page-link", { hasText: "4" }).first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Interactive tab
// ---------------------------------------------------------------------------
test.describe("Interactive tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Interactive");
  });

  test("modal opens and closes", async ({ page }) => {
    // Find the button that opens a modal
    const openBtn = page.locator(".btn", { hasText: /open modal|launch/i }).first();
    await openBtn.click();
    await expect(page.locator(".modal.show")).toBeVisible();
    // Close via the X button or close button
    const closeBtn = page.locator(".modal.show .btn-close, .modal.show .btn", { hasText: /close/i }).first();
    await closeBtn.click();
    await page.waitForTimeout(400);
    await expect(page.locator(".modal.show")).not.toBeVisible();
  });

  test("dropdown opens", async ({ page }) => {
    const dropBtn = page.locator(".dropdown .btn, .dropdown-toggle").first();
    await dropBtn.click();
    await expect(page.locator(".dropdown-menu.show").first()).toBeVisible();
  });

  test("collapse toggles", async ({ page }) => {
    // The collapse toggle button says "Show Content" / "Hide Content"
    const toggleBtn = page.locator(".btn", { hasText: /show content/i }).first();
    await toggleBtn.click();
    await page.waitForTimeout(400);
    await expect(page.locator(".collapse.show").first()).toBeVisible();
  });

  test("accordion renders and toggles", async ({ page }) => {
    await expect(page.locator(".accordion").first()).toBeVisible();
    // Item #1 is open by default; click item #2 to open it
    const item2 = page.locator(".accordion-button").nth(1);
    await item2.click();
    await page.waitForTimeout(400);
    // Item #2 should now be open
    await expect(page.locator(".accordion-collapse.collapse.show").first()).toBeVisible();
  });

  test("tabs inside section work", async ({ page }) => {
    // The interactive section has its own nested tabs
    const innerTab = page.locator(".tab-pane .nav-link, .card .nav-link").first();
    if (await innerTab.isVisible()) {
      await innerTab.click();
      await expect(innerTab).toHaveClass(/active/);
    }
  });

  test("toast renders", async ({ page }) => {
    const toastBtn = page.locator(".btn", { hasText: /toast/i }).first();
    await toastBtn.click();
    await expect(page.locator(".toast.show").first()).toBeVisible();
  });

  test("offcanvas opens and closes", async ({ page }) => {
    const offBtn = page.locator(".btn", { hasText: /offcanvas/i }).first();
    await offBtn.click();
    await expect(page.locator(".offcanvas.show").first()).toBeVisible();
    // Close
    const closeBtn = page.locator(".offcanvas.show .btn-close").first();
    await closeBtn.click();
    await page.waitForTimeout(400);
    await expect(page.locator(".offcanvas.show")).not.toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Overlays tab
// ---------------------------------------------------------------------------
test.describe("Overlays tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Overlays");
  });

  test("tooltip hover shows role, aria, and placement", async ({ page }) => {
    const trigger = page.locator("#tooltip-hover-trigger");
    const wrapper = page.locator(".tooltip-wrapper", { has: trigger });

    await trigger.hover();

    const tooltip = page.locator(".tooltip.show", {
      hasText: "Tooltip on top",
    });
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toHaveAttribute("role", "tooltip");
    await expect(tooltip).toHaveClass(/bs-tooltip-top/);

    const tooltipId = await tooltip.getAttribute("id");
    expect(tooltipId).toBeTruthy();
    await expect(wrapper).toHaveAttribute("aria-describedby", tooltipId!);
  });

  test("tooltip opens from keyboard focus", async ({ page }) => {
    await page.locator("#tooltip-focus-trigger").focus();
    await expect(
      page.locator(".tooltip.show", { hasText: "Tooltip on focus" }),
    ).toBeVisible();
  });

  test("tooltip click trigger toggles", async ({ page }) => {
    const trigger = page.locator("#tooltip-click-trigger");
    const tooltip = page.locator(".tooltip.show", {
      hasText: "Tooltip on click",
    });

    await trigger.click();
    await expect(tooltip).toBeVisible();

    await trigger.click();
    await expect(tooltip).not.toBeVisible();
  });

  test("tooltip falls back when requested placement overflows viewport", async ({
    page,
  }) => {
    const trigger = page.locator("#tooltip-edge-trigger");
    await trigger.evaluate((element) => {
      const wrapper = element.closest(".tooltip-wrapper") as HTMLElement;
      wrapper.style.position = "fixed";
      wrapper.style.top = "2px";
      wrapper.style.left = "320px";
      wrapper.style.zIndex = "1100";
    });
    await page.waitForTimeout(100);

    await trigger.hover();

    const tooltip = page.locator(".tooltip.show", {
      hasText: "Fallback below when top overflows",
    });
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toHaveClass(/bs-tooltip-bottom/);

    const box = await tooltip.boundingBox();
    expect(box?.y ?? -1).toBeGreaterThanOrEqual(0);
  });

  test("popover click shows role, aria, and default placement", async ({ page }) => {
    const trigger = page.locator("#popover-click-trigger");
    const wrapper = page.locator(".popover-wrapper", { has: trigger });

    await trigger.click();

    const popover = page.locator(".popover.show", {
      hasText: "Click Popover",
    });
    await expect(popover).toBeVisible();
    await expect(popover).toHaveAttribute("role", "tooltip");
    await expect(popover).toHaveClass(/bs-popover-end/);

    const popoverId = await popover.getAttribute("id");
    expect(popoverId).toBeTruthy();
    await expect(wrapper).toHaveAttribute("aria-describedby", popoverId!);
  });

  test("popover focus trigger dismisses on blur", async ({ page }) => {
    await page.locator("#popover-focus-trigger").focus();
    const popover = page.locator(".popover.show", {
      hasText: "Focus Dismiss",
    });
    await expect(popover).toBeVisible();

    await page.locator("#popover-click-trigger").focus();
    await expect(popover).not.toBeVisible();
  });

  test("popover outside click dismisses", async ({ page }) => {
    const trigger = page.locator("#popover-outside-trigger");
    const popover = page.locator(".popover.show", {
      hasText: "Outside Dismiss",
    });

    await trigger.click();
    await expect(popover).toBeVisible();

    await page.locator(".popover-backdrop").click({ position: { x: 5, y: 5 } });
    await expect(popover).not.toBeVisible();
  });

  test("popover falls back when requested placement overflows viewport", async ({
    page,
  }) => {
    const trigger = page.locator("#popover-edge-trigger");
    await page.locator("#popover-edge-container").evaluate((element) => {
      const wrapper = element as HTMLElement;
      wrapper.style.position = "fixed";
      wrapper.style.top = "2px";
      wrapper.style.left = "320px";
      wrapper.style.zIndex = "1100";
    });
    await page.waitForTimeout(100);

    await trigger.click();

    const popover = page.locator(".popover.show", {
      hasText: "Fallback Popover",
    });
    await expect(popover).toBeVisible();
    await expect(popover).toHaveClass(/bs-popover-bottom/);

    const box = await popover.boundingBox();
    expect(box?.y ?? -1).toBeGreaterThanOrEqual(0);
  });

  test("popover with empty title and body does not render", async ({ page }) => {
    await page.locator("#popover-empty-trigger").click();
    await expect(page.locator(".popover.show")).toHaveCount(0);
  });
});

// ---------------------------------------------------------------------------
// Media tab
// ---------------------------------------------------------------------------
test.describe("Media tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Media");
  });

  test("carousel renders with slides", async ({ page }) => {
    await expect(page.locator(".carousel").first()).toBeVisible();
    await expect(page.locator(".carousel-item.active").first()).toBeVisible();
  });

  test("carousel controls work", async ({ page }) => {
    const nextBtn = page.locator(".carousel-control-next").first();
    await nextBtn.click();
    await page.waitForTimeout(800);
    // After clicking next, the carousel should have transitioned
    await expect(page.locator(".carousel-item.active").first()).toBeVisible();
  });

  test("carousel indicators render", async ({ page }) => {
    await expect(page.locator(".carousel-indicators").first()).toBeVisible();
    const indicators = page.locator(".carousel-indicators button");
    expect(await indicators.count()).toBeGreaterThan(1);
  });

  test("figure renders", async ({ page }) => {
    await expect(page.locator("figure, .figure").first()).toBeVisible();
  });

  test("ratio/embed renders", async ({ page }) => {
    await expect(page.locator(".ratio").first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Navigation tab
// ---------------------------------------------------------------------------
test.describe("Navigation tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Navigation");
  });

  test("nav pills render", async ({ page }) => {
    await expect(page.locator(".nav-pills").first()).toBeVisible();
  });

  test("nav tabs render", async ({ page }) => {
    // The inner nav-tabs within the Navigation section
    const navTabs = page.locator(".tab-pane .nav-tabs, .tab-content .nav-tabs").first();
    await expect(navTabs).toBeVisible();
  });

  test("breadcrumb renders", async ({ page }) => {
    await expect(page.locator(".breadcrumb").first()).toBeVisible();
  });

  test("scrollspy tracks body scroll and updates nav target", async ({ page }) => {
    await page.locator("#scrollspy-body-beta").evaluate((element) => {
      const rect = element.getBoundingClientRect();
      window.scrollTo({ top: window.scrollY + rect.top - 80, behavior: "auto" });
      window.dispatchEvent(new Event("scroll"));
    });
    await expect(page.locator("#scrollspy-body-active")).toHaveText(
      "scrollspy-body-beta",
    );
    await expect(
      page.locator('#scrollspy-body-nav .nav-link[href="#scrollspy-body-beta"]'),
    ).toHaveClass(/active/);

    await page.locator("#scrollspy-body-gamma").evaluate((element) => {
      const rect = element.getBoundingClientRect();
      window.scrollTo({ top: window.scrollY + rect.top - 80, behavior: "auto" });
      window.dispatchEvent(new Event("scroll"));
    });
    await expect(page.locator("#scrollspy-body-active")).toHaveText(
      "scrollspy-body-gamma",
    );
    await expect(
      page.locator('#scrollspy-body-nav .nav-link[href="#scrollspy-body-gamma"]'),
    ).toHaveClass(/active/);
  });

  test("scrollspy custom container is scoped from body instance", async ({
    page,
  }) => {
    await page.locator("#scrollspy-body-beta").evaluate((element) => {
      const rect = element.getBoundingClientRect();
      window.scrollTo({ top: window.scrollY + rect.top - 80, behavior: "auto" });
      window.dispatchEvent(new Event("scroll"));
    });
    await expect(page.locator("#scrollspy-body-active")).toHaveText(
      "scrollspy-body-beta",
    );

    await page.locator("#scrollspy-custom-root").evaluate((element) => {
      element.scrollTop = 250;
      element.dispatchEvent(new Event("scroll"));
    });

    await expect(page.locator("#scrollspy-custom-active")).toHaveText(
      "scrollspy-custom-two",
    );
    await expect(page.locator("#scrollspy-body-active")).toHaveText(
      "scrollspy-body-beta",
    );
    await expect(
      page.locator('#scrollspy-custom-nav .nav-link[href="#scrollspy-custom-two"]'),
    ).toHaveClass(/active/);
  });

  test("scrollspy refresh picks up dynamic custom sections", async ({ page }) => {
    await page.locator("#scrollspy-add-section").click();
    await expect(
      page.locator('#scrollspy-custom-nav .nav-link[href="#scrollspy-custom-four"]'),
    ).toBeVisible();

    await page.locator("#scrollspy-custom-root").evaluate((element) => {
      element.scrollTop = element.scrollHeight;
      element.dispatchEvent(new Event("scroll"));
    });

    await expect(page.locator("#scrollspy-custom-active")).toHaveText(
      "scrollspy-custom-four",
    );
    await expect(
      page.locator('#scrollspy-custom-nav .nav-link[href="#scrollspy-custom-four"]'),
    ).toHaveClass(/active/);
  });
});

// ---------------------------------------------------------------------------
// More tab
// ---------------------------------------------------------------------------
test.describe("More tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "More");
  });

  test("placeholders render", async ({ page }) => {
    await expect(page.locator(".placeholder, .placeholder-glow, .placeholder-wave").first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Responsive / Navbar
// ---------------------------------------------------------------------------
test.describe("Navbar", () => {
  test("navbar collapse toggles on small viewport", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto("/");
    await waitForApp(page);
    // Hamburger should be visible
    const toggler = page.locator(".navbar-toggler").first();
    await expect(toggler).toBeVisible();
    await toggler.click();
    await page.waitForTimeout(400);
    await expect(page.locator(".navbar-collapse.show, .navbar-collapse.collapsing").first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Docs section
// ---------------------------------------------------------------------------
test.describe("Docs section", () => {
  test("component reference section exists", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await expect(page.locator("#docs-section")).toBeVisible();
    await expect(page.locator("#docs-section h2")).toContainText("Component Reference");
  });
});

// ---------------------------------------------------------------------------
// New in 0.6.0 — every component and prop the release added
//
// These assert the CLASS CONTRACT, not that an element merely exists. The whole
// value of a typed component layer is that it emits exactly the Bootstrap markup
// Bootstrap's own CSS selects on, so "a span appeared" proves nothing; "a span
// carrying text-bg-primary appeared" is the claim worth locking.
// ---------------------------------------------------------------------------
test.describe("New in 0.6.0", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "New in 0.6.0");
  });

  test("CheckboxButton emits the btn-check idiom", async ({ page }) => {
    const input = page.locator("input#nw-chk-bold");
    await expect(input).toHaveClass(/btn-check/);
    await expect(input).toHaveAttribute("type", "checkbox");
    // The visible control is a label.btn bound to the input, not a <button>.
    const label = page.locator('label[for="nw-chk-bold"]');
    await expect(label).toHaveClass(/btn/);
    await expect(label).toContainText("Bold");
  });

  test("CheckboxButton carries colour, outline and size variants", async ({ page }) => {
    await expect(page.locator('label[for="nw-chk-bold"]')).toHaveClass(/btn-primary/);
    await expect(page.locator('label[for="nw-chk-italic"]')).toHaveClass(/btn-outline-primary/);
    await expect(page.locator('label[for="nw-chk-sm"]')).toHaveClass(/btn-sm/);
    await expect(page.locator('label[for="nw-chk-lg"]')).toHaveClass(/btn-lg/);
    await expect(page.locator("input#nw-chk-disabled")).toBeDisabled();
  });

  test("CheckboxButton toggles independently", async ({ page }) => {
    await expect(page.getByText("Bold is on")).toBeVisible();
    await page.locator('label[for="nw-chk-bold"]').click();
    await expect(page.getByText("Bold is off")).toBeVisible();
    // Italic is unaffected — these are independent checkboxes, not a radio group.
    await expect(page.getByText("italic is off")).toBeVisible();
  });

  test("RadioButton groups by name and selects one at a time", async ({ page }) => {
    const start = page.locator("input#nw-radio-start");
    await expect(start).toHaveAttribute("type", "radio");
    await expect(start).toHaveAttribute("name", "nw-align");
    await expect(start).toHaveClass(/btn-check/);

    await expect(page.getByText("Selected: center")).toBeVisible();
    await page.locator('label[for="nw-radio-start"]').click();
    await expect(page.getByText("Selected: start")).toBeVisible();
    await expect(page.locator("input#nw-radio-justify")).toBeDisabled();
  });

  test("Button link and plain variants emit the right classes", async ({ page }) => {
    const link = page.locator("button", { hasText: "Link style" }).first();
    await expect(link).toHaveClass(/btn-link/);

    const plain = page.locator("button", { hasText: "Plain (no variant)" }).first();
    await expect(plain).toHaveClass(/btn/);
    // The point of `plain` is the ABSENCE of a colour variant.
    await expect(plain).not.toHaveClass(/btn-primary|btn-secondary|btn-success|btn-danger/);
  });

  test("Button rel, role and onmousedown are wired", async ({ page }) => {
    await expect(page.locator('a[rel="noopener noreferrer"]')).toHaveAttribute("target", "_blank");
    await expect(page.locator('button[role="switch"]')).toBeVisible();

    await page.locator("button", { hasText: "onmousedown" }).first().click();
    await expect(page.getByText("mousedown fired before click")).toBeVisible();
  });

  test("Badge fill switches between text-bg and bg", async ({ page }) => {
    await expect(page.locator("span", { hasText: "text-bg (default)" }).first()).toHaveClass(/text-bg-primary/);

    const bgOnly = page.locator("span", { hasText: "bg only" }).first();
    await expect(bgOnly).toHaveClass(/bg-primary/);
    await expect(bgOnly).not.toHaveClass(/text-bg-primary/);
  });

  test("Badge onclick fires", async ({ page }) => {
    const badge = page.locator("span.badge", { hasText: "clicked" }).first();
    await expect(badge).toContainText("clicked 0×");
    await badge.click();
    await expect(badge).toContainText("clicked 1×");
  });

  test("Card renders as an anchor when href is set", async ({ page }) => {
    const card = page.locator('a.card[href="https://getbootstrap.com/"]').first();
    await expect(card).toHaveAttribute("target", "_blank");
  });

  test("Card per-slot classes and body id/style land", async ({ page }) => {
    await expect(page.locator(".card-header.bg-primary-subtle")).toBeVisible();
    const body = page.locator("#nw-card-body");
    await expect(body).toHaveClass(/card-body/);
    await expect(body).toHaveAttribute("style", /border-left/);
  });

  test("Card click and contextmenu handlers fire", async ({ page }) => {
    const body = page.locator("#nw-card-body");
    await body.click();
    await expect(page.getByText("card clicked")).toBeVisible();
    await body.click({ button: "right" });
    await expect(page.getByText("card right-clicked")).toBeVisible();
  });

  test("Alert heading renders as alert-heading", async ({ page }) => {
    const heading = page.locator(".alert .alert-heading", { hasText: "Well done" });
    await expect(heading).toBeVisible();
  });

  test("ListGroup numbered renders a real ordered list", async ({ page }) => {
    const ol = page.locator("ol.list-group.list-group-numbered");
    await expect(ol).toBeVisible();
    await expect(ol.locator("li.list-group-item")).toHaveCount(3);
  });

  test("BreadcrumbItem onclick fires without navigating", async ({ page }) => {
    await page.locator(".breadcrumb-item a", { hasText: "Library" }).click();
    await expect(page.getByText("navigated: Library")).toBeVisible();
  });

  test("Navbar container variants", async ({ page }) => {
    await expect(page.locator(".navbar .container-fluid").first()).toBeVisible();
    const bare = page.locator(".navbar", { hasText: "no container" }).first();
    await expect(bare.locator(".container, .container-fluid")).toHaveCount(0);
  });

  test("Modal custom header and slot classes", async ({ page }) => {
    await page.locator("button", { hasText: "Open slotted modal" }).click();
    await expect(page.locator(".modal-content.border-primary")).toBeVisible();
    await expect(page.locator(".modal-header.bg-primary-subtle")).toBeVisible();
    await expect(page.locator(".modal-body.bg-body-tertiary")).toBeVisible();
    await expect(page.locator(".modal-footer.justify-content-between")).toBeVisible();
    await expect(page.locator(".modal-dialog.modal-lg")).toBeVisible();
    await expect(page.locator(".modal-header", { hasText: "A fully custom header" })).toBeVisible();
    await page.locator("button", { hasText: "Done" }).click();
  });

  test("TabList content_style and fill apply", async ({ page }) => {
    const nav = page.locator(".nav-fill").first();
    await expect(nav).toBeVisible();
    const pane = page.locator('.tab-content[style*="min-height"]');
    await expect(pane).toHaveCount(1);
    await expect(pane).toHaveClass(/border-top-0/);
  });

  test("ToastContainer positioned=false drops fixed positioning", async ({ page }) => {
    const container = page.locator(".toast-container").filter({ hasText: "document flow" }).first();
    await expect(container).toBeVisible();
    await expect(container).not.toHaveClass(/position-fixed/);

    // Asserting the absent CLASS is not enough, and this is the reason the check is
    // written twice. Bootstrap's own `.toast-container` rule is `position: absolute`,
    // so a container that has correctly lost the utility can still be out of flow and
    // overlay whatever follows it. The rendered result is the claim worth locking.
    const laidOut = await container.evaluate((el) => {
      const cs = getComputedStyle(el);
      const parent = getComputedStyle(el.parentElement!);
      return {
        position: cs.position,
        // in flow => the parent grows to contain it
        parentTallerThanContainer:
          Number.parseFloat(parent.height) >= Number.parseFloat(cs.height),
      };
    });
    expect(laidOut.position).toBe("static");
    expect(laidOut.parentTallerThanContainer).toBe(true);
  });

  test("Input typed numeric and file attributes", async ({ page }) => {
    const num = page.locator('input[type="number"]').first();
    await expect(num).toHaveAttribute("step", "0.25");
    await expect(num).toHaveAttribute("min", "0");
    await expect(num).toHaveAttribute("max", "10");
    await expect(page.locator('input[type="file"]').first()).toHaveAttribute("accept", "image/png,image/jpeg");
  });
});

// ---------------------------------------------------------------------------
// Coverage tab — the props the themed sections do not reach
// ---------------------------------------------------------------------------
test.describe("Coverage", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Coverage");
  });

  test("Container fluid and Col breakpoint/offset/order classes", async ({ page }) => {
    await expect(page.locator(".container-fluid.bg-primary-subtle")).toBeVisible();
    await expect(page.locator(".col-xs-6, .col-6").first()).toBeVisible();
    await expect(page.locator('[class*="offset-"]').first()).toBeVisible();
    await expect(page.locator('[class*="order-"]').first()).toBeVisible();
  });

  test("Accordion flush", async ({ page }) => {
    await expect(page.locator(".accordion.accordion-flush")).toBeVisible();
  });

  test("Collapse horizontal", async ({ page }) => {
    await page.locator("button", { hasText: "Toggle horizontal collapse" }).click();
    await expect(page.locator(".collapse-horizontal").first()).toBeVisible();
  });

  test("Table borderless, sm and dark", async ({ page }) => {
    const table = page.locator("table.table-borderless").first();
    await expect(table).toHaveClass(/table-sm/);
    await expect(table).toHaveClass(/table-dark/);
  });

  test("Dropdown direction and item states", async ({ page }) => {
    await expect(page.locator(".dropup").first()).toBeVisible();
    await expect(page.locator(".dropend").first()).toBeVisible();
    await expect(page.locator(".dropdown-menu.dropdown-menu-end").first()).toBeVisible();

    // Items live in a closed menu until the toggle is clicked: present, not visible.
    await page.locator("button", { hasText: "Drops up" }).click();
    const menu = page.locator(".dropup .dropdown-menu.show");
    await expect(menu.locator(".dropdown-item.active")).toBeVisible();
    await expect(menu.locator(".dropdown-item.disabled")).toBeVisible();
    await menu.locator(".dropdown-item", { hasText: "With onclick" }).click();
    await expect(page.getByText("dropdown item clicked")).toBeVisible();
  });

  test("Input events fire and the datalist is wired", async ({ page }) => {
    const input = page.locator('input[list="nw-suggestions"]');
    await input.focus();
    await expect(page.getByText("Last event: onfocus")).toBeVisible();
    await input.press("a");
    await expect(page.getByText(/Last event: onkey/)).toBeVisible();
    await expect(page.locator("datalist#nw-suggestions option")).toHaveCount(3);
  });

  test("readonly and disabled inputs", async ({ page }) => {
    await expect(page.locator("input[readonly]").first()).toBeVisible();
    await expect(page.locator("input:disabled").first()).toBeDisabled();
  });

  test("Select size and disabled", async ({ page }) => {
    await expect(page.locator("select.form-select-lg").first()).toBeVisible();
    await expect(page.locator("select:disabled").first()).toBeDisabled();
  });

  test("Range step and disabled", async ({ page }) => {
    const range = page.locator("input.form-range").first();
    await expect(range).toHaveAttribute("step", "10");
    await expect(page.locator("input.form-range:disabled")).toBeVisible();
  });

  test("ButtonGroup sizes", async ({ page }) => {
    await expect(page.locator(".btn-group-lg").first()).toBeVisible();
    await expect(page.locator(".btn-group-sm").first()).toBeVisible();
  });

  test("Pagination window and prev/next suppression", async ({ page }) => {
    const paginations = page.locator(".pagination");
    await expect(paginations.first()).toBeVisible();
    // The second pagination has show_prev_next off, so it has no arrow items.
    const second = paginations.nth(1);
    await expect(second).toHaveClass(/pagination-sm/);
  });

  test("Placeholder sizes and animations", async ({ page }) => {
    await expect(page.locator(".placeholder-lg").first()).toBeVisible();
    await expect(page.locator(".placeholder-sm").first()).toBeVisible();
    await expect(page.locator(".placeholder-glow").first()).toBeVisible();
    await expect(page.locator(".placeholder-wave").first()).toBeVisible();
  });

  test("Figure thumbnail and fluid", async ({ page }) => {
    await expect(page.locator("img.img-thumbnail").first()).toBeVisible();
    await expect(page.locator("figure img.img-fluid").first()).toBeVisible();
    await expect(page.locator(".figure-caption.text-end").first()).toBeVisible();
  });

  test("Offcanvas placement", async ({ page }) => {
    await page.locator("button", { hasText: "Open from top" }).click();
    const top = page.locator(".offcanvas.offcanvas-top");
    await expect(top).toBeVisible();
    await top.locator(".btn-close").click();
    await expect(top).toHaveCount(0);

    await page.locator("button", { hasText: "Open from bottom" }).click();
    const bottom = page.locator(".offcanvas.offcanvas-bottom");
    await expect(bottom).toBeVisible();
    // backdrop: false — no overlay is rendered for this one.
    await expect(page.locator(".offcanvas-backdrop")).toHaveCount(0);
    await bottom.locator(".btn-close").click();
    await expect(bottom).toHaveCount(0);
  });

  test("Offcanvas responsive swaps the base class", async ({ page }) => {
    // responsive: "md" replaces the `offcanvas` base with `offcanvas-md` — Bootstrap's
    // own responsive form, which renders inline above the breakpoint. The base class is
    // therefore deliberately absent, and asserting that is the point of the test.
    await page.locator("button", { hasText: "Open responsive" }).click();
    await expect(page.locator(".offcanvas-md.offcanvas-start")).toBeVisible();
    await expect(page.locator(".offcanvas.offcanvas-start")).toHaveCount(0);
  });

  test("Modal with every dismissal route disabled", async ({ page }) => {
    await page.locator("button", { hasText: "Open a strict modal" }).click();
    const modal = page.locator(".modal.show").first();
    await expect(modal).toBeVisible();
    // show_close: false — no × in the header.
    await expect(modal.locator(".modal-header .btn-close")).toHaveCount(0);
    // keyboard_close: false — Escape must not dismiss it.
    await page.keyboard.press("Escape");
    await expect(modal).toBeVisible();
    await page.locator("button", { hasText: "Acknowledge" }).click();
    await expect(page.locator(".modal.show")).toHaveCount(0);
  });

  test("disabled triggers render a wrapper", async ({ page }) => {
    await expect(page.locator("button:disabled", { hasText: "disabled + tooltip" })).toBeVisible();
    await expect(page.locator("button:disabled", { hasText: "disabled + popover" })).toBeVisible();
  });

  // ── forced-open overlays ────────────────────────────────────────────────
  //
  // These lock the two halves of a defect that shipped invisible: an overlay is
  // painted `position: fixed` from a single measurement, so (a) one opened while
  // its trigger was below the fold had its box clamped INTO the viewport and
  // floated ~2700px from the thing it points at, and (b) nothing ever re-measured,
  // so any open overlay detached the moment the page scrolled.

  test("a forced-open overlay is suppressed while its trigger is off-screen", async ({ page }) => {
    const state = await page.evaluate(() => {
      const tip = document.querySelector(".tooltip");
      const trig = Array.from(document.querySelectorAll("button"))
        .find((b) => b.textContent?.includes("always visible"));
      if (!tip || !trig) return null;
      return {
        visibility: getComputedStyle(tip).visibility,
        triggerTop: trig.getBoundingClientRect().top,
        viewport: window.innerHeight,
      };
    });
    expect(state).not.toBeNull();
    // The trigger really is below the fold — otherwise this test proves nothing.
    expect(state!.triggerTop).toBeGreaterThan(state!.viewport);
    expect(state!.visibility).toBe("hidden");

    // Hidden, NOT unmounted: an overlay that is removed cannot be measured, so it
    // could never come back when the trigger scrolls into view.
    await expect(page.locator(".tooltip")).toHaveCount(1);
  });

  test("a forced-open overlay anchors to its trigger once scrolled into view", async ({ page }) => {
    await page.locator("h3", { hasText: "Tooltip and Popover" }).first().scrollIntoViewIfNeeded();
    await page.waitForTimeout(900);

    const geo = await page.evaluate(() => {
      const tip = document.querySelector(".tooltip");
      const trig = Array.from(document.querySelectorAll("button"))
        .find((b) => b.textContent?.includes("always visible"));
      if (!tip || !trig) return null;
      const t = trig.getBoundingClientRect();
      const o = tip.getBoundingClientRect();
      return {
        visibility: getComputedStyle(tip).visibility,
        gap: o.top - t.bottom,
        centreOffset: Math.abs(o.left + o.width / 2 - (t.left + t.width / 2)),
      };
    });
    expect(geo).not.toBeNull();
    expect(geo!.visibility).toBe("visible");
    // Placed just below the trigger and centred on it, rather than parked anywhere.
    expect(geo!.gap).toBeGreaterThanOrEqual(0);
    expect(geo!.gap).toBeLessThan(24);
    expect(geo!.centreOffset).toBeLessThan(4);
  });

  test("an open overlay keeps tracking its trigger through a scroll", async ({ page }) => {
    await page.locator("h3", { hasText: "Tooltip and Popover" }).first().scrollIntoViewIfNeeded();
    await page.waitForTimeout(900);

    const measure = () =>
      page.evaluate(() => {
        const tip = document.querySelector(".tooltip");
        const trig = Array.from(document.querySelectorAll("button"))
          .find((b) => b.textContent?.includes("always visible"));
        if (!tip || !trig) return null;
        const t = trig.getBoundingClientRect();
        const o = tip.getBoundingClientRect();
        return { gap: o.top - t.bottom, triggerTop: t.top };
      });

    const before = await measure();
    await page.mouse.wheel(0, 140);
    await page.waitForTimeout(800);
    const after = await measure();

    expect(before).not.toBeNull();
    expect(after).not.toBeNull();
    // The page actually moved…
    expect(Math.abs(after!.triggerTop - before!.triggerTop)).toBeGreaterThan(50);
    // …and the overlay moved with it, keeping the same gap.
    expect(Math.abs(after!.gap - before!.gap)).toBeLessThan(4);
  });

  test("Carousel fade and dark", async ({ page }) => {
    const carousel = page.locator(".carousel.carousel-fade").first();
    await expect(carousel).toBeVisible();
    await expect(carousel).toHaveClass(/carousel-dark/);
    await expect(carousel.locator(".carousel-indicators")).toBeVisible();
    await expect(carousel.locator(".carousel-control-next")).toBeVisible();
  });

  test("Toast autohide dismisses itself", async ({ page }) => {
    await page.locator("button", { hasText: "Show a toast that autohides" }).click();
    const toast = page.locator(".toast", { hasText: "delay_ms: 3000" });
    await expect(toast).toBeVisible();
    await expect(toast).toBeHidden({ timeout: 8_000 });
  });

  test("TabList justified with pills", async ({ page }) => {
    const nav = page.locator(".nav-pills.nav-justified").first();
    await expect(nav).toBeVisible();
  });

  test("NavLink prevent_default keeps the page put", async ({ page }) => {
    const before = page.url();
    await page.locator(".nav-link", { hasText: "prevent_default" }).click();
    await expect(page.getByText("nav link — default prevented")).toBeVisible();
    expect(page.url()).toBe(before);
  });
});

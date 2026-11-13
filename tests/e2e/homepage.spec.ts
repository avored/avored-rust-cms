import { expect, test } from "@playwright/test";

test("homepage displays the Avored Rust CMS label", async ({ page }) => {
  const stylesheetResponsePromise = page.waitForResponse((response) =>
    response.url().includes("/public/pkg/avored-rust-cms.css"),
  );
  await page.goto("/");

  const stylesheetResponse = await stylesheetResponsePromise;
  expect(stylesheetResponse.ok()).toBe(true);
  expect(await stylesheetResponse.text()).toContain(".bg-gray-100");
  await expect(page.getByText(/^Avored Rust CMS$/i)).toBeVisible();
});

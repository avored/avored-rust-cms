import { expect, test } from "@playwright/test";

test("homepage displays the Avored Rust CMS label", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByText(/^Avored Rust CMS$/i)).toBeVisible();
});

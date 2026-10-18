interface AdminProfile {
  id: string;
  name: string;
  email: string;
}

interface AppLayoutState {
  isSidebarCollapsed: boolean;
  user: AdminProfile;
  profileUpdatedListener: (() => void) | null;
  init(): void;
  restoreProfile(): Promise<void>;
  destroy(): void;
  toggleSidebar(): void;
}

function readStoredToken(): string {
  const value = localStorage.getItem("auth_token");
  if (!value) return "";

  try {
    const parsed: unknown = JSON.parse(value);
    return typeof parsed === "string" ? parsed : value;
  } catch {
    return value;
  }
}

function readStoredProfile(): AdminProfile | null {
  const value = localStorage.getItem("auth_user");
  if (!value) return null;

  try {
    const parsed: unknown = JSON.parse(value);
    if (
      typeof parsed === "object" &&
      parsed !== null &&
      "id" in parsed &&
      "name" in parsed &&
      "email" in parsed &&
      typeof parsed.id === "string" &&
      typeof parsed.name === "string" &&
      typeof parsed.email === "string"
    ) {
      return { id: parsed.id, name: parsed.name, email: parsed.email };
    }
    throw new Error("Stored administrator profile has an invalid shape.");
  } catch (error) {
    localStorage.removeItem("auth_user");
    console.error("Unable to restore administrator profile from local storage.", error);
    return null;
  }
}

export function appLayout() {
  return {
    isSidebarCollapsed: false,
    user: { id: "", name: "", email: "" },
    profileUpdatedListener: null,

    init() {
      this.user = readStoredProfile() ?? this.user;
      this.profileUpdatedListener = () => {
        this.user = readStoredProfile() ?? { id: "", name: "", email: "" };
      };
      window.addEventListener("auth-profile-updated", this.profileUpdatedListener);
      void this.restoreProfile();
    },

    async restoreProfile() {
      const token = readStoredToken();
      if (!token.trim()) {
        localStorage.removeItem("auth_user");
        window.location.assign("/auth/login");
        return;
      }

      try {
        const response = await fetch("/api/v1/admin/me", {
          headers: {
            Accept: "application/json",
            Authorization: `Bearer ${token}`,
          },
        });

        if (response.status === 401) {
          localStorage.removeItem("auth_token");
          localStorage.removeItem("auth_user");
          this.user = { id: "", name: "", email: "" };
          window.location.assign("/auth/login");
          return;
        }
        if (!response.ok) {
          throw new Error(`Admin profile request failed with HTTP ${response.status}.`);
        }

        const profile: unknown = await response.json();
        if (
          typeof profile !== "object" ||
          profile === null ||
          !("id" in profile) ||
          !("name" in profile) ||
          !("email" in profile) ||
          typeof profile.id !== "string" ||
          typeof profile.name !== "string" ||
          typeof profile.email !== "string"
        ) {
          throw new Error("Admin profile response has an invalid shape.");
        }

        this.user = { id: profile.id, name: profile.name, email: profile.email };
        localStorage.setItem("auth_user", JSON.stringify(this.user));
      } catch (error) {
        console.error("Unable to load the administrator profile.", error);
      }
    },

    destroy() {
      if (this.profileUpdatedListener) {
        window.removeEventListener("auth-profile-updated", this.profileUpdatedListener);
      }
    },

    toggleSidebar() {
      this.isSidebarCollapsed = !this.isSidebarCollapsed;
    },
  } satisfies AppLayoutState;
}

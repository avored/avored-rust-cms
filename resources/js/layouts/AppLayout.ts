export function appLayout() {
  return {
    isSidebarCollapsed: false,

    toggleSidebar() {
      this.isSidebarCollapsed = !this.isSidebarCollapsed;
    },
  };
}

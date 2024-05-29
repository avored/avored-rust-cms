export const useLoggedInUser = (): string | null => {
  const user = localStorage.getItem("AUTH_ADMIN_USER");
  try {
    return user ? JSON.parse(user) : null;
  } catch (error) {
    console.error("Error parsing user data", error);
    return null;
  }
};

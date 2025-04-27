export type LoginPostType = {
    email: string;
    password: string;
}

export type ForgotPasswordPostType = {
    email: string;
}

export type ResetPasswordPostType = {
    email: string;
    password: string;
    confirm_password: string;
    token: string;
}
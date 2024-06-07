export default interface IResetPasswordPost {
    email: string;
    password: string;
    confirm_password: string;
    token: string;
}
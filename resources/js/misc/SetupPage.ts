import http, { HttpError } from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';

export function setupPage() {
    return {
        ...formErrorsMixin(),

        name: 'Admin',
        email: 'admin@avored.com',
        password: 'password1234',
        confirmPassword: 'password1234',
        submitting: false,

        async handleSubmit() {
            if (this.password !== this.confirmPassword) {
                this.errorMessage = 'Passwords do not match';
                return;
            }

            this.submitting = true;
            this.clearErrors();

            try {
                await http.post('/api/misc/setup', {
                    name: this.name,
                    email: this.email,
                    password: this.password,
                    confirm_password: this.confirmPassword,
                });

                window.location.href = '/auth/login';
            } catch (err: any) {
                this.applyApiErrors(err);
            } finally {
                this.submitting = false;
            }
        },
    };
}

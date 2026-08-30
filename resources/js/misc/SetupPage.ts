import http from '../utils/http';

export function setupPage() {
    return {
        name: '',
        email: '',
        password: '',
        confirmPassword: '',
        submitting: false,
        errorMessage: '',

        async handleSubmit() {
            if (this.password !== this.confirmPassword) {
                this.errorMessage = 'Passwords do not match';
                return;
            }

            this.submitting = true;
            this.errorMessage = '';

            const res = await http.post('/api/setup', {
                name: this.name,
                email: this.email,
                password: this.password,
                confirm_password: this.confirmPassword,
            });

            this.submitting = false;

            if (res.ok) {
                window.location.href = '/admin/login';
            } else {
                this.errorMessage = res.error || 'Setup failed';
            }
        },
    };
}

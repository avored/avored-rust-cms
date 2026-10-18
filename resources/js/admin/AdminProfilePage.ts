import http from '../utils/http';

export function adminProfilePage() {
    return {
        activeTab: 'profile', // 'profile' | 'password'
        loading: false,
        submittingProfile: false,
        submittingPassword: false,
        errorMessage: '',
        successMessage: '',
        initialized: false,

        profileForm: {
            name: '',
            email: '',
        },

        passwordForm: {
            current_password: '',
            new_password: '',
            confirm_password: '',
        },

        async init() {
            if (this.initialized) return;
            this.initialized = true;
            await this.fetchProfile();
        },

        async fetchProfile() {
            this.loading = true;
            this.errorMessage = '';
            try {
                const data = await http.get('/api/v1/admin/me');
                this.profileForm.name = data.name || '';
                this.profileForm.email = data.email || '';
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to fetch admin profile details.';
            } finally {
                this.loading = false;
            }
        },

        async submitProfile() {
            this.errorMessage = '';
            this.successMessage = '';

            if (!this.profileForm.name.trim()) {
                this.errorMessage = 'Name is required.';
                return;
            }
            if (!this.profileForm.email.trim()) {
                this.errorMessage = 'Email is required.';
                return;
            }

            this.submittingProfile = true;
            try {
                await http.put('/api/v1/admin/me', {
                    name: this.profileForm.name.trim(),
                    email: this.profileForm.email.trim(),
                });
                this.successMessage = 'Profile updated successfully.';

                try {
                    const profile = await http.get('/api/v1/admin/me');
                    this.profileForm.name = profile.name || '';
                    this.profileForm.email = profile.email || '';
                    localStorage.setItem('auth_user', JSON.stringify({
                        id: profile.id,
                        name: profile.name,
                        email: profile.email,
                    }));
                    window.dispatchEvent(new Event('auth-profile-updated'));
                } catch (err: any) {
                    this.errorMessage = `Profile was saved, but the refreshed profile could not be synchronized: ${err.message || 'request failed'}`;
                }
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to update profile.';
            } finally {
                this.submittingProfile = false;
            }
        },

        async submitPassword() {
            this.errorMessage = '';
            this.successMessage = '';

            if (!this.passwordForm.current_password) {
                this.errorMessage = 'Current password is required.';
                return;
            }
            if (!this.passwordForm.new_password) {
                this.errorMessage = 'New password is required.';
                return;
            }
            if (this.passwordForm.new_password.length < 6) {
                this.errorMessage = 'New password must be at least 6 characters.';
                return;
            }
            if (this.passwordForm.new_password !== this.passwordForm.confirm_password) {
                this.errorMessage = 'New password and confirm password do not match.';
                return;
            }

            this.submittingPassword = true;
            try {
                const res = await http.put('/api/v1/admin/me/password', {
                    current_password: this.passwordForm.current_password,
                    new_password: this.passwordForm.new_password,
                    confirm_password: this.passwordForm.confirm_password,
                });
                this.successMessage = res.message || 'Password updated successfully.';
                this.passwordForm.current_password = '';
                this.passwordForm.new_password = '';
                this.passwordForm.confirm_password = '';
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to update password.';
            } finally {
                this.submittingPassword = false;
            }
        },
    };
}

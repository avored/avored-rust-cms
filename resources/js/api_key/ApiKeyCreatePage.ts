import http from '../utils/http';

export function apiKeyCreatePage() {
    return {
        name: '',
        permissions: {
            'GET:/api/v1/entities': false,
            'POST:/api/v1/entities': false,
            'GET:/api/v1/entities/{id}': false,
            'PATCH:/api/v1/entities/{id}': false,
            'DELETE:/api/v1/entities/{id}': false,
        } as Record<string, boolean>,
        submitting: false,
        errorMessage: '',
        plainSecret: null as string | null,
        copied: false,

        selectedPermissions() {
            return Object.entries(this.permissions)
                .filter(([_, v]) => v)
                .map(([k]) => {
                    const [method, endpoint] = k.split(':');
                    return { method, endpoint };
                });
        },

        async submit() {
            if (!this.name.trim()) {
                this.errorMessage = 'Name is required.';
                return;
            }

            this.submitting = true;
            this.errorMessage = '';
            try {
                const response = await http.post('/api/api-keys', {
                    name: this.name.trim(),
                    permissions: this.selectedPermissions(),
                });
                this.plainSecret = response.plain_secret || null;
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to create API key';
            } finally {
                this.submitting = false;
            }
        },

        async copySecret() {
            if (!this.plainSecret) return;
            await navigator.clipboard.writeText(this.plainSecret);
            this.copied = true;
            setTimeout(() => {
                this.copied = false;
            }, 2500);
        },
    };
}

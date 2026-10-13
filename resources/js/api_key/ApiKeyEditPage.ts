import http from '../utils/http';

export function apiKeyEditPage(apiKeyId: string = '') {
    return {
        id: apiKeyId,
        name: '',
        keyPrefix: '',
        permissions: {
            'GET:/api/v1/entities': false,
            'POST:/api/v1/entities': false,
            'GET:/api/v1/entities/{id}': false,
            'PATCH:/api/v1/entities/{id}': false,
            'DELETE:/api/v1/entities/{id}': false,
        } as Record<string, boolean>,
        loading: false,
        submitting: false,
        deleteModalOpen: false,
        deleting: false,
        errorMessage: '',
        successMessage: '',
        initialized: false,

        async init() {
            if (this.initialized) return;
            this.initialized = true;

            if (!this.id) {
                // Fallback: parse ID from URL e.g. /admin/api-keys/:id/edit
                const parts = window.location.pathname.split('/').filter(Boolean);
                const editIdx = parts.indexOf('edit');
                if (editIdx > 0 && parts[editIdx - 1]) {
                    this.id = parts[editIdx - 1] ?? '';
                }
            }

            if (this.id) {
                await this.fetchKey();
            }
        },

        async fetchKey() {
            this.loading = true;
            this.errorMessage = '';
            try {
                const key = await http.get(`/api/api-keys/${this.id}`);
                this.name = key.name || '';
                this.keyPrefix = key.key_prefix || '';

                if (Array.isArray(key.permissions)) {
                    for (const perm of key.permissions) {
                        const entry = `${perm.method}:${perm.endpoint}`;
                        if (entry in this.permissions) {
                            this.permissions[entry] = true;
                        }
                    }
                }
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to load API key';
            } finally {
                this.loading = false;
            }
        },

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
            this.successMessage = '';
            try {
                await http.put(`/api/api-keys/${this.id}`, {
                    name: this.name.trim(),
                    permissions: this.selectedPermissions(),
                });
                this.successMessage = 'API key updated successfully.';
                setTimeout(() => {
                    this.successMessage = '';
                }, 3000);
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to update API key';
            } finally {
                this.submitting = false;
            }
        },

        confirmDelete() {
            this.deleteModalOpen = true;
        },

        cancelDelete() {
            this.deleteModalOpen = false;
        },

        async deleteKey() {
            if (!this.id || this.deleting) return;
            this.deleting = true;
            try {
                await http.delete(`/api/api-keys/${this.id}`);
                window.location.href = '/admin/api-keys';
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to delete API key';
                this.deleting = false;
            }
        },
    };
}

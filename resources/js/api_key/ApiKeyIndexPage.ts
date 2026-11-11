import http from '../utils/http';
import { PaginatedResponse } from '../types/EntityType';

export function apiKeyIndexPage() {
    return {
        keys: [] as Array<Record<string, any>>,
        total: 0,
        page: 1,
        limit: 25,
        lastPage: 1,
        loading: false,
        deleteModalOpen: false,
        keyToDelete: null as Record<string, any> | null,
        deleting: false,
        errorMessage: '',
        initialized: false,

        async init() {
            if (this.initialized) return;
            this.initialized = true;
            await this.fetchKeys();
        },

        async fetchKeys() {
            this.loading = true;
            this.errorMessage = '';
            try {
                const response = await http.get<PaginatedResponse<Record<string, any>>>('/api/v1/admin/api-keys', {
                    params: {
                        page: this.page,
                        per_page: this.limit,
                    },
                });

                this.keys = response.data || [];
                this.total = response.pagination.total;
                this.lastPage = response.pagination.last_page;
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to load API keys';
                this.keys = [];
                this.total = 0;
                this.lastPage = 1;
            } finally {
                this.loading = false;
            }
        },

        totalPages() {
            return this.lastPage;
        },

        firstVisibleItem() {
            return this.total === 0 ? 0 : (this.page - 1) * this.limit + 1;
        },

        lastVisibleItem() {
            return Math.min(this.page * this.limit, this.total);
        },

        async previousPage() {
            if (this.page <= 1 || this.loading) return;
            this.page -= 1;
            await this.fetchKeys();
        },

        async nextPage() {
            if (this.page >= this.totalPages() || this.loading) return;
            this.page += 1;
            await this.fetchKeys();
        },

        confirmDelete(key: Record<string, any>) {
            this.keyToDelete = key;
            this.deleteModalOpen = true;
        },

        cancelDelete() {
            this.deleteModalOpen = false;
            this.keyToDelete = null;
        },

        async deleteKey() {
            if (!this.keyToDelete?.id || this.deleting) return;
            this.deleting = true;
            try {
                await http.delete(`/api/v1/admin/api-keys/${this.keyToDelete.id}`);
                this.cancelDelete();
                await this.fetchKeys();
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to delete API key';
            } finally {
                this.deleting = false;
            }
        },
    };
}

import http from '../utils/http';
import { EntityInterface, EntityPaginationResponse } from './types';

export function entityIndexPage() {
    return {
        entities: [] as EntityInterface[],
        total: 0,
        page: 1,
        pageSize: 20,
        loading: false,
        deleteModalOpen: false,
        entityToDelete: null as EntityInterface | null,
        deleting: false,
        errorMessage: '',

        async init() {
            await this.fetchEntities();
        },

        async fetchEntities() {
            this.loading = true;
            this.errorMessage = '';
            try {
                const response = await http.get<EntityPaginationResponse>('/api/entities', {
                    params: {
                        page: this.page,
                        page_size: this.pageSize,
                    },
                });
                this.entities = response.data || [];
                this.total = response.total || 0;
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to load entities';
            } finally {
                this.loading = false;
            }
        },

        totalPages() {
            return Math.max(1, Math.ceil(this.total / this.pageSize));
        },

        firstVisibleItem() {
            return this.total === 0 ? 0 : (this.page - 1) * this.pageSize + 1;
        },

        lastVisibleItem() {
            return Math.min(this.page * this.pageSize, this.total);
        },

        async previousPage() {
            if (this.page <= 1 || this.loading) return;
            this.page -= 1;
            await this.fetchEntities();
        },

        async nextPage() {
            if (this.page >= this.totalPages() || this.loading) return;
            this.page += 1;
            await this.fetchEntities();
        },

        formatDate(value: string) {
            if (!value) return '-';

            const date = new Date(value);
            return Number.isNaN(date.getTime())
                ? value
                : new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' }).format(date);
        },

        confirmDelete(entity: EntityInterface) {
            this.entityToDelete = entity;
            this.deleteModalOpen = true;
        },

        cancelDelete() {
            this.deleteModalOpen = false;
            this.entityToDelete = null;
        },

        async deleteEntity() {
            if (!this.entityToDelete) return;
            this.deleting = true;
            try {
                await http.delete(`/api/entities/${this.entityToDelete.id}`);
                this.entities = this.entities.filter(e => e.id !== this.entityToDelete?.id);
                this.total = Math.max(0, this.total - 1);
                this.deleteModalOpen = false;
                this.entityToDelete = null;

                if (this.entities.length === 0 && this.page > 1) {
                    this.page -= 1;
                    await this.fetchEntities();
                }
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to delete entity';
            } finally {
                this.deleting = false;
            }
        },
    };
}

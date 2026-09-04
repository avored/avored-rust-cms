import http from '../utils/http';
import { EntityInterface, EntityPaginationResponse } from './types';

export function entityIndexPage() {
    return {
        entities: [] as EntityInterface[],
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
                const response = await http.get<EntityPaginationResponse>('/api/entity');
                this.entities = response.data || [];
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to load entities';
            } finally {
                this.loading = false;
            }
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
                await http.delete(`/api/entity/${this.entityToDelete.id}`);
                this.entities = this.entities.filter(e => e.id !== this.entityToDelete?.id);
                this.deleteModalOpen = false;
                this.entityToDelete = null;
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to delete entity';
            } finally {
                this.deleting = false;
            }
        },
    };
}

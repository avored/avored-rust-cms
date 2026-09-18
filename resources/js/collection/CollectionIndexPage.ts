import { entityApi } from "../services/EntityApi";
import { EntityInterface, EntityOptionInterface } from "../types/EntityType";

export function collectionIndexPage(entity_id: string) {
    return {
        entitiesOptions: [] as Array<EntityOptionInterface>,
        selectedEntityId: entity_id,
        currentEntity: null as EntityInterface | null,
        collections: [] as Array<Record<string, any>>,
        total: 0,
        page: 1,
        pageSize: 20,
        loading: false,
        errorMessage: "",

        async init() {
            this.entitiesOptions = await entityApi.fetchEntitiesOptions() ?? [];
            if (this.selectedEntityId) {
                await this.loadCollections();
            }
        },

        async loadCollections() {
            if (!this.selectedEntityId) return;

            this.loading = true;
            this.errorMessage = "";
            try {
                const response = await entityApi.fetchCollectionsByEntityId(
                    this.selectedEntityId,
                    this.page,
                    this.pageSize
                );
                if (response) {
                    this.currentEntity = response.entity;
                    this.collections = response.data || [];
                    this.total = response.total || 0;
                }
            } catch (err: any) {
                this.errorMessage = err.message || "Failed to load collection data";
                this.collections = [];
                this.total = 0;
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
            await this.loadCollections();
        },

        async nextPage() {
            if (this.page >= this.totalPages() || this.loading) return;
            this.page += 1;
            await this.loadCollections();
        },

        getAttributeValue(item: Record<string, any>, identifier: string): string {
            const val = item[identifier];
            if (val === undefined || val === null) return "-";
            if (typeof val === "object") return JSON.stringify(val);
            return String(val);
        }
    };
}

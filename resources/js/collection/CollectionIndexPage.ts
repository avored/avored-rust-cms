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
        deleteModalOpen: false,
        recordToDelete: null as Record<string, any> | null,
        deleting: false,
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
                    this.collections = (response.data || []).map((item: Record<string, any>) => ({ ...item }));
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

        normalizeSurrealValue(value: any): any {
            if (value === null || value === undefined) return value;

            if (Array.isArray(value)) {
                return value.map((item) => this.normalizeSurrealValue(item));
            }

            if (typeof value !== "object") {
                return value;
            }

            const keys = Object.keys(value);
            if (keys.length === 1) {
                const key = keys[0];
                switch (key) {
                    case "String":
                        return value.String ?? "";
                    case "Bool":
                        return Boolean(value.Bool);
                    case "Number":
                        return Number(value.Number ?? 0);
                    case "Null":
                        return null;
                    case "RecordId": {
                        const record = value.RecordId;
                        const keyValue = record?.key?.String ?? record?.key ?? "";
                        return `${record?.table ?? ""}:${keyValue}`;
                    }
                    case "Array":
                        return this.normalizeSurrealValue(value.Array ?? []);
                    case "Object":
                        return this.normalizeSurrealValue(value.Object ?? {});
                    default:
                        break;
                }
            }

            return value;
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

        confirmDelete(record: Record<string, any>) {
            this.recordToDelete = record;
            this.deleteModalOpen = true;
        },

        cancelDelete() {
            this.deleteModalOpen = false;
            this.recordToDelete = null;
        },

        async deleteRecord() {
            if (!this.recordToDelete || !this.selectedEntityId) return;

            const recordId = this.getAttributeValue(this.recordToDelete, 'id');
            if (!recordId || recordId === '-') return;

            this.deleting = true;
            try {
                await entityApi.deleteCollection(this.selectedEntityId, recordId);
                this.collections = this.collections.filter((item) => this.getAttributeValue(item, 'id') !== recordId);
                this.total = Math.max(0, this.total - 1);
                this.deleteModalOpen = false;
                this.recordToDelete = null;
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to delete collection record';
            } finally {
                this.deleting = false;
            }
        },

        getAttributeValue(item: Record<string, any>, identifier: string): string {
            const rawValue = item?.[identifier];

            if (identifier === "id") {
                const recordId = rawValue?.RecordId;
                const keyValue = recordId?.key?.String ?? recordId?.key ?? "";
                return keyValue || "-";
            }

            const val = this.normalizeSurrealValue(rawValue);
            if (val === undefined || val === null) return "-";
            if (typeof val === "object") return JSON.stringify(val);
            return String(val);
        }
    };
}

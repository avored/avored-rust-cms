import { entityApi } from "../services/EntityApi";
import { EntityInterface, EntityOptionInterface } from "../types/EntityType";

export function collectionEditPage(entity_id: string, record_id: string) {
    return {
        entitiesOptions: [] as Array<EntityOptionInterface>,
        selectedEntityId: entity_id,
        recordId: record_id,
        currentEntity: null as EntityInterface | null,
        formData: {} as Record<string, any>,
        loading: false,
        submitting: false,
        errorMessage: "",

        async init() {
            this.entitiesOptions = (await entityApi.fetchEntitiesOptions()) ?? [];
            if (this.selectedEntityId) {
                await this.loadEntity();
            }
        },

        async loadEntity() {
            if (!this.selectedEntityId || !this.recordId) return;

            this.loading = true;
            this.errorMessage = "";

            try {
                const entity = await entityApi.fetchEntityById(this.selectedEntityId);
                this.currentEntity = entity;
                const record = await entityApi.fetchCollectionById(this.selectedEntityId, this.recordId);
                this.formData = {};

                if (entity?.attributes) {
                    for (const attribute of entity.attributes) {
                        const value = record?.[attribute.identifier];
                        this.formData[attribute.identifier] = this.normalizeFormValue(value, attribute);
                    }
                }
            } catch (err: any) {
                this.errorMessage = err.message || "Failed to load record details";
                this.currentEntity = null;
            } finally {
                this.loading = false;
            }
        },

        normalizeFormValue(value: any, attribute: any) {
            if (value === null || value === undefined) {
                if (attribute.field_type === "checkbox") return false;
                if (attribute.data_type === "integer" || attribute.data_type === "float") return 0;
                return "";
            }

            if (attribute.field_type === "checkbox") {
                return Boolean(value);
            }

            if (attribute.data_type === "integer" || attribute.data_type === "float") {
                return Number(value);
            }

            if (typeof value === "object" && value !== null) {
                const keys = Object.keys(value);
                if (keys.length === 1 && value[keys[0]] !== undefined && typeof value[keys[0]] !== "object") {
                    return value[keys[0]];
                }
                return JSON.stringify(value);
            }

            return String(value);
        },

        async submit() {
            if (!this.selectedEntityId || !this.recordId) {
                this.errorMessage = "Please select an entity and record first.";
                return;
            }

            this.submitting = true;
            this.errorMessage = "";

            try {
                await entityApi.updateCollection(this.selectedEntityId, this.recordId, { ...this.formData });
                window.location.href = `/admin/collections?entity_id=${this.selectedEntityId}`;
            } catch (err: any) {
                this.errorMessage = err.message || "Failed to update collection record";
            } finally {
                this.submitting = false;
            }
        },
    };
}

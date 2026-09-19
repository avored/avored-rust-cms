import { entityApi } from "../services/EntityApi";
import { EntityInterface, EntityOptionInterface } from "../types/EntityType";

export function collectionCreatePage(entity_id: string) {
    return {
        entitiesOptions: [] as Array<EntityOptionInterface>,
        selectedEntityId: entity_id,
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
            if (!this.selectedEntityId) return;

            this.loading = true;
            this.errorMessage = "";
            try {
                const entity = await entityApi.fetchEntityById(this.selectedEntityId);
                this.currentEntity = entity;
                this.formData = {};
                if (entity?.attributes) {
                    for (const attribute of entity.attributes) {
                        const key = attribute.identifier;
                        if (attribute.field_type === "checkbox") {
                            this.formData[key] = false;
                        } else if (attribute.data_type === "integer" || attribute.data_type === "float") {
                            this.formData[key] = 0;
                        } else if (attribute.data_type === "date") {
                            this.formData[key] = "";
                        } else {
                            this.formData[key] = "";
                        }
                    }
                }
            } catch (err: any) {
                this.errorMessage = err.message || "Failed to load entity details";
                this.currentEntity = null;
            } finally {
                this.loading = false;
            }
        },

        async submit() {
            if (!this.selectedEntityId) {
                this.errorMessage = "Please select an entity first.";
                return;
            }

            this.submitting = true;
            this.errorMessage = "";

            try {
                const payload = { ...this.formData };
                const result = await entityApi.createCollection(this.selectedEntityId, payload);
                if (result) {
                    window.location.href = `/admin/collections?entity_id=${this.selectedEntityId}`;
                }
            } catch (err: any) {
                this.errorMessage = err.message || "Failed to create collection record";
            } finally {
                this.submitting = false;
            }
        },
    };
}

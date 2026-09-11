import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';
import { EntityInterface } from '../types/EntityType';

export function entityEditPage(entityId: string) {
    return {
        ...formErrorsMixin(),

        id: entityId,
        name: '',
        identifier: '',
        originalIdentifier: '',
        identifierEnabled: false,
        identifierSubmitting: false,
        loading: false,
        submitting: false,
        attributes: [] as Array<{
            id: string;
            name: string;
            identifier: string;
            data_type: string;
            field_type: string;
            is_new: boolean;
        }>,
        attributeModalOpen: false,
        editingAttributeId: null as string | null,
        editingAttributeIsNew: false,
        attributeIdentifierTouched: false,
        attributeDraft: {
            name: '',
            identifier: '',
            data_type: 'string',
            field_type: 'text',
        },
       

        async init() {
            await this.fetchEntity();
        },

        async fetchEntity() {
            this.loading = true;
            this.clearErrors();
            try {
                const entity = await http.get<EntityInterface>(`/api/entities/${this.id}`);
                this.name = entity.name || '';
                this.identifier = entity.identifier || '';
                this.originalIdentifier = this.identifier;
                this.identifierEnabled = false;
                this.attributes = entity.attributes.map(a => ({
                    id: a.id,
                    name: a.name,
                    identifier: a.identifier,
                    data_type: a.data_type,
                    field_type: a.field_type,
                    is_new: false,
                }))

                console.log(this.attributes)
            } catch (err: any) {
                this.applyApiErrors(err);
            } finally {
                this.loading = false;
            }
        },

        enableIdentifier() {
            this.identifierEnabled = true;
        },

        cancelIdentifierEdit() {
            this.identifier = this.originalIdentifier;
            this.identifierEnabled = false;
            this.clearErrors();
        },

        async saveIdentifier() {
            this.identifierSubmitting = true;
            this.clearErrors();

            try {
                await http.put(`/api/entities/${this.id}/identifier`, {
                    identifier: this.identifier,
                });

                this.originalIdentifier = this.identifier;
                this.identifierEnabled = false;
            } catch (err: any) {
                this.applyApiErrors(err);
            } finally {
                this.identifierSubmitting = false;
            }
        },

        async handleSubmit() {
            this.submitting = true;
            this.clearErrors();

            const payloadAttributes = this.attributes.map(a => {
                return {
                    name: a.name,
                    identifier: a.identifier,
                    data_type: a.data_type,
                    field_type: a.field_type,
                    is_new: a.is_new,
                }
            })

            try {
                await http.put(`/api/entities/${this.id}`, {
                    name: this.name,
                    attributes: payloadAttributes
                });

                window.location.href = '/admin/entity';
            } catch (err: any) {
                this.applyApiErrors(err);
            } finally {
                this.submitting = false;
            }
        },
        submitAttribute() {
            const name = this.attributeDraft.name.trim();
            const identifier = this.attributeDraft.identifier.trim();

            if (!name || !identifier) {
                this.errorMessage = 'Attribute name and identifier are required.';
                return;
            }

            const attributePayload = {
                name,
                identifier,
                data_type: this.attributeDraft.data_type,
                field_type: this.attributeDraft.field_type,
            };

            if (this.editingAttributeId !== null) {
                this.attributes = this.attributes.map((attribute) => {
                    if (attribute.id === this.editingAttributeId) {
                        return { ...attribute, ...attributePayload };
                    }
                    return attribute;
                });
            } else {
                this.attributes.push({
                    id: crypto.randomUUID(),
                    is_new: true,
                    ...attributePayload,
                });
            }

            this.closeAttributeModal();
        },

        removeAttribute(attributeId: string) {
            this.attributes = this.attributes.filter((attribute) => attribute.id !== attributeId);
        },
        openAttributeModal(attributeId: string | null) {
            this.attributeModalOpen = true;
            this.clearErrors();

            const attribute = attributeId !== null
                ? this.attributes.find((a) => a.id === attributeId)
                : undefined;

            if (attribute) {
                this.editingAttributeId = attribute.id;
                this.editingAttributeIsNew = attribute.is_new;
                this.attributeDraft = {
                    name: attribute.name,
                    identifier: attribute.identifier,
                    data_type: attribute.data_type,
                    field_type: attribute.field_type,
                };
                this.attributeIdentifierTouched = true;
                return;
            }

            this.editingAttributeId = null;
            this.editingAttributeIsNew = true;
            this.attributeIdentifierTouched = false;
            this.attributeDraft = {
                name: '',
                identifier: '',
                data_type: 'string',
                field_type: 'text',
            };
        },

        closeAttributeModal() {
            this.attributeModalOpen = false;
            this.editingAttributeId = null;
            this.editingAttributeIsNew = false;
            this.attributeIdentifierTouched = false;
            this.attributeDraft = {
                name: '',
                identifier: '',
                data_type: 'string',
                field_type: 'text',
            };
            this.clearErrors();
        },

        handleAttributeNameChange() {
            if (!this.attributeIdentifierTouched) {
                this.attributeDraft.identifier = this.attributeDraft.name
                    .toLowerCase()
                    .replace(/[^a-z0-9]+/g, '_')
                    .replace(/^_+|_+$/g, '');
            }
        },

        handleAttributeIdentifierInput() {
            this.attributeIdentifierTouched = true;
        },
    };
}

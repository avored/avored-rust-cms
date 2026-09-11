import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';

export function entityCreatePage() {
    return {
        ...formErrorsMixin(),

        name: '',
        identifier: '',
        submitting: false,
        identifierTouched: false,
        attributeModalOpen: false,
        editingAttributeId: null as number | null,
        attributeIdentifierTouched: false,
        attributeDraft: {
            name: '',
            identifier: '',
            data_type: 'string',
            field_type: 'text',
        },
        attributes: [] as Array<{
            id: number;
            name: string;
            identifier: string;
            data_type: string;
            field_type: string;
        }>,

        handleNameChange() {
            if (!this.identifierTouched) {
                this.identifier = this.name
                    .toLowerCase()
                    .replace(/[^a-z0-9]+/g, '_')
                    .replace(/^_+|_+$/g, '');
            }
        },

        handleIdentifierInput() {
            this.identifierTouched = true;
        },

        openAttributeModal(attribute: any = null) {
            this.attributeModalOpen = true;
            this.clearErrors();

            if (attribute) {
                this.editingAttributeId = attribute.id;
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
                        return {
                            ...attribute,
                            ...attributePayload,
                        };
                    }

                    return attribute;
                });
            } else {
                this.attributes.push({
                    id: Date.now() + Math.random(),
                    ...attributePayload,
                });
            }

            this.closeAttributeModal();
        },

        removeAttribute(attributeId: number) {
            this.attributes = this.attributes.filter((attribute) => attribute.id !== attributeId);
        },

        async handleSubmit() {
            this.submitting = true;
            this.clearErrors();

            try {
                await http.post('/api/entities', {
                    name: this.name,
                    identifier: this.identifier,
                });

                window.location.href = '/admin/entity';
            } catch (err: any) {
                this.applyApiErrors(err);
            } finally {
                this.submitting = false;
            }
        },
    };
}

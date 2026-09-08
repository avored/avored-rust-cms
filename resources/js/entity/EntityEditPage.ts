import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';
import { EntityInterface } from './types';

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

            try {
                await http.put(`/api/entities/${this.id}`, {
                    name: this.name,
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

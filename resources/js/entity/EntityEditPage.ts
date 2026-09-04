import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';
import { EntityInterface } from './types';

export function entityEditPage(entityId: string) {
    return {
        ...formErrorsMixin(),

        id: entityId,
        name: '',
        identifier: '',
        data_type: 'TEXT',
        loading: false,
        submitting: false,

        async init() {
            await this.fetchEntity();
        },

        async fetchEntity() {
            this.loading = true;
            this.clearErrors();
            try {
                const entity = await http.get<EntityInterface>(`/api/entity/${this.id}`);
                this.name = entity.name || '';
                this.identifier = entity.identifier || '';
                this.data_type = entity.data_type || 'TEXT';
            } catch (err: any) {
                this.applyApiErrors(err);
            } finally {
                this.loading = false;
            }
        },

        async handleSubmit() {
            this.submitting = true;
            this.clearErrors();

            try {
                await http.put(`/api/entity/${this.id}`, {
                    name: this.name,
                    identifier: this.identifier,
                    data_type: this.data_type,
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

import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';
import { EntityInterface } from './types';

export function entityEditPage(entityId: string) {
    return {
        ...formErrorsMixin(),

        id: entityId,
        name: '',
        identifier: '',
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
                await http.put(`/api/entities/${this.id}`, {
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

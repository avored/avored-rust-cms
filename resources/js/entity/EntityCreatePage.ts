import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';

export function entityCreatePage() {
    return {
        ...formErrorsMixin(),

        name: '',
        identifier: '',
        submitting: false,
        identifierTouched: false,

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

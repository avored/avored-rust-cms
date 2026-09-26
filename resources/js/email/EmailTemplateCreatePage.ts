import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';

export function emailTemplateCreatePage() {
    return {
        ...formErrorsMixin(),

        name: '',
        subject: '',
        body_html: '<h1>Hello {{ user.first_name }}</h1>',
        body_plain: 'Hello {{ user.first_name }}',
        submitting: false,

        async handleSubmit() {
            this.submitting = true;
            this.clearErrors();

            try {
                await http.post('/api/email-templates', {
                    name: this.name,
                    subject: this.subject,
                    body_html: this.body_html,
                    body_plain: this.body_plain,
                });

                window.location.href = '/admin/email-templates';
            } catch (err: any) {
                this.applyApiErrors(err, 'Failed to create email template');
            } finally {
                this.submitting = false;
            }
        },
    };
}

import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';

export function emailTemplateEditPage(templateId: string) {
    return {
        ...formErrorsMixin(),

        id: templateId,
        name: '',
        subject: '',
        body_html: '',
        body_plain: '',
        loading: false,
        submitting: false,

        async init() {
            await this.fetchTemplate();
        },

        async fetchTemplate() {
            this.loading = true;
            this.clearErrors();

            try {
                const template = await http.get<Record<string, any>>(`/api/email-templates/${this.id}`);
                this.name = template.name || '';
                this.subject = template.subject || '';
                this.body_html = template.body_html || '';
                this.body_plain = template.body_plain || '';
            } catch (err: any) {
                this.applyApiErrors(err, 'Failed to load email template');
            } finally {
                this.loading = false;
            }
        },

        async handleSubmit() {
            this.submitting = true;
            this.clearErrors();

            try {
                await http.put(`/api/email-templates/${this.id}`, {
                    name: this.name,
                    subject: this.subject,
                    body_html: this.body_html,
                    body_plain: this.body_plain,
                });

                window.location.href = '/admin/email-templates';
            } catch (err: any) {
                this.applyApiErrors(err, 'Failed to update email template');
            } finally {
                this.submitting = false;
            }
        },
    };
}

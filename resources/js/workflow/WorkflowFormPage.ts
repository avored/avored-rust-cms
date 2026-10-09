import http from '../utils/http';
import { formErrorsMixin } from '../utils/formErrors';

function createWorkflowAction(type: 'webhook' | 'email' = 'webhook') {
    if (type === 'webhook') {
        return {
            type,
            url: '',
        };
    }

    return {
        type,
        template_id: '',
        recipient: '',
    };
}

export function workflowFormPage(workflowId: string = '') {
    return {
        ...formErrorsMixin(),

        id: workflowId,
        name: '',
        triggerEntityType: '',
        triggerEvent: 'created',
        isActive: true,
        actions: [createWorkflowAction()],
        loading: false,
        submitting: false,

        async init() {
            if (this.id) {
                await this.fetchWorkflow();
            }
        },

        addAction() {
            this.actions.push(createWorkflowAction());
        },

        removeAction(index: number) {
            if (this.actions.length <= 1) return;
            this.actions.splice(index, 1);
        },

        buildPayload() {
            return {
                name: this.name.trim(),
                trigger: {
                    entity_type: this.triggerEntityType.trim(),
                    event: this.triggerEvent,
                },
                is_active: this.isActive,
                actions: this.actions.map((action: Record<string, any>) => {
                    if (action.type === 'email') {
                        return {
                            type: 'email',
                            template_id: String(action.template_id || '').trim(),
                            recipient: String(action.recipient || '').trim(),
                        };
                    }

                    return {
                        type: 'webhook',
                        url: String(action.url || '').trim(),
                    };
                }),
            };
        },

        validateForm() {
            this.clearErrors();

            const hasName = this.name.trim().length > 0;
            if (!hasName) {
                this.fieldErrors.name = 'Workflow name is required.';
            }

            const hasEntityType = this.triggerEntityType.trim().length > 0;
            if (!hasEntityType) {
                this.fieldErrors['trigger.entity_type'] = 'Trigger entity type is required.';
            }

            if (!['created', 'updated'].includes(this.triggerEvent)) {
                this.fieldErrors['trigger.event'] = 'Trigger event is required.';
            }

            if (!Array.isArray(this.actions) || this.actions.length === 0) {
                this.fieldErrors.actions = 'At least one action is required.';
            }

            this.actions.forEach((action: Record<string, any>, index: number) => {
                if (action.type === 'webhook') {
                    if (!String(action.url || '').trim()) {
                        this.fieldErrors[`actions[${index}].url`] = 'Webhook URL is required.';
                    }
                    return;
                }

                if (action.type === 'email') {
                    if (!String(action.template_id || '').trim()) {
                        this.fieldErrors[`actions[${index}].template_id`] = 'Template ID is required.';
                    }
                    if (!String(action.recipient || '').trim()) {
                        this.fieldErrors[`actions[${index}].recipient`] = 'Recipient is required.';
                    }
                    return;
                }

                this.fieldErrors[`actions[${index}]`] = 'Unsupported action type.';
            });

            return Object.keys(this.fieldErrors).length === 0;
        },

        async fetchWorkflow() {
            this.loading = true;
            this.clearErrors();

            try {
                const workflow = await http.get<Record<string, any>>(`/api/workflows/${this.id}`);
                this.name = workflow.name || '';
                this.triggerEntityType = workflow.trigger?.entity_type || '';
                this.triggerEvent = workflow.trigger?.event || 'created';
                this.isActive = Boolean(workflow.is_active);
                this.actions = Array.isArray(workflow.actions) && workflow.actions.length > 0
                    ? workflow.actions.map((action: Record<string, any>) => {
                        if (action.type === 'email') {
                            return {
                                type: 'email',
                                template_id: action.template_id || '',
                                recipient: action.recipient || '',
                            };
                        }

                        return {
                            type: 'webhook',
                            url: action.url || '',
                        };
                    })
                    : [createWorkflowAction()];
            } catch (err: any) {
                this.applyApiErrors(err, 'Failed to load workflow');
            } finally {
                this.loading = false;
            }
        },

        async submit() {
            if (!this.validateForm()) {
                return;
            }

            this.submitting = true;
            this.clearErrors();

            try {
                const payload = this.buildPayload();

                if (this.id) {
                    await http.put(`/api/workflows/${this.id}`, payload);
                } else {
                    await http.post('/api/workflows', payload);
                }

                window.location.href = '/admin/workflows';
            } catch (err: any) {
                this.applyApiErrors(err, 'Failed to save workflow');
            } finally {
                this.submitting = false;
            }
        },
    };
}

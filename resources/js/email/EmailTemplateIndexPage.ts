import http from '../utils/http';
import { PaginatedResponse } from '../types/EntityType';

export function emailTemplateIndexPage() {
    return {
        templates: [] as Array<Record<string, any>>,
        total: 0,
        page: 1,
        pageSize: 25,
        lastPage: 1,
        loading: false,
        deleteModalOpen: false,
        templateToDelete: null as Record<string, any> | null,
        deleting: false,
        errorMessage: '',
        initialized: false,

        async init() {
            if (this.initialized) return;
            this.initialized = true;
            await this.fetchTemplates();
        },

        async fetchTemplates() {
            this.loading = true;
            this.errorMessage = '';
            try {
                const response = await http.get<PaginatedResponse<Record<string, any>>>('/api/v1/admin/email-templates', {
                    params: {
                        page: this.page,
                        per_page: this.pageSize,
                    },
                });

                this.templates = response.data || [];
                this.total = response.pagination.total;
                this.lastPage = response.pagination.last_page;
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to load email templates';
                this.templates = [];
                this.total = 0;
                this.lastPage = 1;
            } finally {
                this.loading = false;
            }
        },

        totalPages() {
            return this.lastPage;
        },

        firstVisibleItem() {
            return this.total === 0 ? 0 : (this.page - 1) * this.pageSize + 1;
        },

        lastVisibleItem() {
            return Math.min(this.page * this.pageSize, this.total);
        },

        async previousPage() {
            if (this.page <= 1 || this.loading) return;
            this.page -= 1;
            await this.fetchTemplates();
        },

        async nextPage() {
            if (this.page >= this.totalPages() || this.loading) return;
            this.page += 1;
            await this.fetchTemplates();
        },

        formatDate(value: string) {
            if (!value) return '-';
            const date = new Date(value);
            return Number.isNaN(date.getTime())
                ? value
                : new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' }).format(date);
        },

        confirmDelete(template: Record<string, any>) {
            this.templateToDelete = template;
            this.deleteModalOpen = true;
        },

        cancelDelete() {
            this.deleteModalOpen = false;
            this.templateToDelete = null;
        },

        async deleteTemplate() {
            if (!this.templateToDelete) return;

            this.deleting = true;
            try {
                await http.delete(`/api/v1/admin/email-templates/${this.templateToDelete.id}`);
                this.templates = this.templates.filter((template) => template.id !== this.templateToDelete?.id);
                this.total = Math.max(0, this.total - 1);
                this.deleteModalOpen = false;
                this.templateToDelete = null;

                if (this.templates.length === 0 && this.page > 1) {
                    this.page -= 1;
                    await this.fetchTemplates();
                }
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to delete email template';
            } finally {
                this.deleting = false;
            }
        },
    };
}

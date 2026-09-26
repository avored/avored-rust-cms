import http from '../utils/http';

export function emailTemplateIndexPage() {
    return {
        templates: [] as Array<Record<string, any>>,
        total: 0,
        page: 1,
        pageSize: 20,
        loading: false,
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
                const response = await http.get('/api/email-templates', {
                    params: {
                        page: this.page,
                        page_size: this.pageSize,
                    },
                });

                this.templates = response.data || [];
                this.total = response.total || 0;
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to load email templates';
                this.templates = [];
                this.total = 0;
            } finally {
                this.loading = false;
            }
        },

        totalPages() {
            return Math.max(1, Math.ceil(this.total / this.pageSize));
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
    };
}

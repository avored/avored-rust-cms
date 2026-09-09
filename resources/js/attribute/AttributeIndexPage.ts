import http from '../utils/http';
import { AttributeInterface, AttributePaginationResponse } from '../types/AttributeType';

export function attributeIndexPage() {
    return {
        attributes: [] as AttributeInterface[],
        total: 0,
        page: 1,
        pageSize: 20,
        loading: false,
        deleteModalOpen: false,
        attributeToDelete: null as AttributeInterface | null,
        deleting: false,
        errorMessage: '',

        async init() {
            await this.fetchAttributes();
        },

        async fetchAttributes() {
            this.loading = true;
            this.errorMessage = '';
            try {
                const response = await http.get<AttributePaginationResponse>('/api/attributes', {
                    params: {
                        page: this.page,
                        page_size: this.pageSize,
                    },
                });
                this.attributes = response.data || [];
                this.total = response.total || 0;
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to load attributes';
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
            await this.fetchAttributes();
        },

        async nextPage() {
            if (this.page >= this.totalPages() || this.loading) return;
            this.page += 1;
            await this.fetchAttributes();
        },

        formatDate(value: string) {
            if (!value) return '-';

            const date = new Date(value);
            return Number.isNaN(date.getTime())
                ? value
                : new Intl.DateTimeFormat(undefined, { dateStyle: 'medium' }).format(date);
        },

        confirmDelete(attribute: AttributeInterface) {
            this.attributeToDelete = attribute;
            this.deleteModalOpen = true;
        },

        cancelDelete() {
            this.deleteModalOpen = false;
            this.attributeToDelete = null;
        },

        async deleteAttribute() {
            if (!this.attributeToDelete) return;
            this.deleting = true;
            try {
                await http.delete(`/api/attributes/${this.attributeToDelete.id}`);
                this.attributes = this.attributes.filter(e => e.id !== this.attributeToDelete?.id);
                this.total = Math.max(0, this.total - 1);
                this.deleteModalOpen = false;
                this.attributeToDelete = null;

                if (this.attributes.length === 0 && this.page > 1) {
                    this.page -= 1;
                    await this.fetchAttributes();
                }
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to delete attribute';
            } finally {
                this.deleting = false;
            }
        },
    };
}

import http from '../utils/http';

export interface GeneralSettings {
    site_name: string;
    logo_url: string;
    support_email: string;
    timezone: string;
    default_language: string;
}

export interface SeoSettings {
    meta_title: string;
    meta_description: string;
    og_image_url: string;
}

export interface EmailSettings {
    smtp_host: string;
    smtp_port: string;
    sender_name: string;
    default_reply_to: string;
}


export interface SettingsState {
    general: GeneralSettings;
    seo: SeoSettings;
    email: EmailSettings;
}

export function settingsPage() {
    return {
        activeTab: 'general', // 'general' | 'seo' | 'email'
        loading: false,
        saving: false,
        errorMessage: '',
        successMessage: '',
        initialized: false,
        isDirty: false,
        originalDataJson: '',

        form: {
            general: {
                site_name: '',
                logo_url: '',
                support_email: '',
                timezone: 'UTC',
                default_language: 'en',
            },
            seo: {
                meta_title: '',
                meta_description: '',
                og_image_url: '',
            },
            email: {
                smtp_host: '',
                smtp_port: '587',
                sender_name: '',
                default_reply_to: '',
            }
        } as SettingsState,

        async init() {
            if (this.initialized) return;
            this.initialized = true;

            await this.fetchSettings();

            window.addEventListener('beforeunload', (e) => {
                if (this.isDirty) {
                    e.preventDefault();
                    e.returnValue = '';
                }
            });
        },

        markDirty() {
            this.isDirty = true;
        },

        async fetchSettings() {
            this.loading = true;
            this.errorMessage = '';
            try {
                const data = await http.get('/api/v1/settings');
                if (data) {
                    this.form = {
                        general: { ...this.form.general, ...(data.general || {}) },
                        seo: { ...this.form.seo, ...(data.seo || {}) },
                        email: { ...this.form.email, ...(data.email || {}) },
                    };
                    this.originalDataJson = JSON.stringify(this.form);
                    this.isDirty = false;
                }
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to fetch settings.';
            } finally {
                this.loading = false;
            }
        },

        discardChanges() {
            if (this.originalDataJson) {
                this.form = JSON.parse(this.originalDataJson);
                this.isDirty = false;
                this.successMessage = 'Changes discarded.';
                setTimeout(() => {
                    this.successMessage = '';
                }, 3000);
            }
        },

        async saveSettings() {
            this.errorMessage = '';
            this.successMessage = '';

            // Basic validation
            if (!this.form.general.site_name.trim()) {
                this.activeTab = 'general';
                this.errorMessage = 'Site name is required in General settings.';
                return;
            }
            if (!this.form.general.support_email.trim()) {
                this.activeTab = 'general';
                this.errorMessage = 'Support email is required in General settings.';
                return;
            }

            this.saving = true;
            try {
                const updated = await http.put('/api/v1/settings', this.form);
                if (updated) {
                    this.form = {
                        general: { ...this.form.general, ...(updated.general || {}) },
                        seo: { ...this.form.seo, ...(updated.seo || {}) },
                        email: { ...this.form.email, ...(updated.email || {}) },
                    };
                    this.originalDataJson = JSON.stringify(this.form);
                    this.isDirty = false;
                    this.successMessage = 'Settings saved successfully.';
                    setTimeout(() => {
                        this.successMessage = '';
                    }, 4000);
                }
            } catch (err: any) {
                this.errorMessage = err.message || 'Failed to save settings.';
            } finally {
                this.saving = false;
            }
        },
    };
}

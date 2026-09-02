import { HttpError } from './http';

/**
 * Generic API validation error shape returned by the Rust backend.
 *
 * Example response (HTTP 400):
 *   { "status": false, "errors": [{ "key": "name", "message": "validation_required" }] }
 */
export interface ApiError {
    key: string;
    message: string;
}

export interface ApiErrorResponse {
    status: boolean;
    errors?: ApiError[];
}

/**
 * Parses the errors array from an API error response into a plain
 * key → message map.  Returns an empty object when there are no
 * field-level errors.
 *
 * @param data - The parsed JSON body of a non-OK response.
 */
export function parseApiErrors(data: unknown): Record<string, string> {

    if (typeof data === 'string') {
        data = JSON.parse(data);
    }

    if (!data || typeof data !== 'object') return {};

    const body = data as ApiErrorResponse;
    if (!Array.isArray(body.errors) || body.errors.length === 0) return {};

    return body.errors.reduce<Record<string, string>>((map, err) => {
        if (err.key) map[err.key] = err.message ?? '';
        return map;
    }, {});
}

/**
 * Reusable Alpine.js mixin that any page component can spread into its
 * data object.  Tracks per-field validation errors and a general error
 * message, and exposes helpers consumed by the template.
 *
 * Usage:
 *   import { formErrorsMixin } from '../utils/formErrors';
 *
 *   export function myPage() {
 *       return {
 *           ...formErrorsMixin(),
 *           // ... page-specific state
 *       };
 *   }
 *
 * Template helpers (Alpine.js):
 *   fieldError('name')          – returns the error string for that field, or ''
 *   hasFieldError('name')       – boolean, drives x-bind:class / x-if
 *   applyApiErrors(responseData) – call with res.data when res.ok is false
 */
export function formErrorsMixin() {
    return {
        errorMessage: '' as string,
        fieldErrors: {} as Record<string, string>,

        /** Returns the validation message for a field, or empty string. */
        fieldError(field: string): string {
            return this.fieldErrors[field] ?? '';
        },

        /** True when the field has a validation error. */
        hasFieldError(field: string): boolean {
            return Boolean(this.fieldErrors[field]);
        },

        /** Clears all errors (call before each submission). */
        clearErrors() {
            this.errorMessage = '';
            this.fieldErrors = {};
        },

        /**
         * Applies errors from a caught HttpError (or any unknown throw).
         * If the error body contains field-level validation errors they
         * populate `fieldErrors`; otherwise `errorMessage` is set.
         *
         * Usage in a catch block:
         *   } catch (err) { this.applyApiErrors(err); }
         *
         * @param err      - The caught value (ideally an HttpError).
         * @param fallback - Message used when no structured errors are found.
         */
        applyApiErrors(err: unknown, fallback = 'Request failed') {
            if (err instanceof HttpError) {
                const parsed = parseApiErrors(err.data);
               
                if (Object.keys(parsed).length > 0) {
                    console.log('test');
                    this.fieldErrors = parsed;
                    return;
                }
                // Use the HttpError message if no field errors (e.g. 401, 500)
                this.errorMessage = err.message || fallback;
            } else {
                // Network error or unexpected throw
                this.errorMessage = (err instanceof Error ? err.message : null) || fallback;
            }
        },
    };
}

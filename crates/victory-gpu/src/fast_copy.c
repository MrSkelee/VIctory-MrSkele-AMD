#include <excpt.h>
#include <stddef.h>
#include <string.h>

/*
 * Fast in-process memory copy guarded by structured exception handling.
 * Returns 1 on complete copy, or 0 if an access violation / in-page error
 * hits an unmapped or unreadable page so the caller can fall back cleanly.
 */
int victory_try_copy(void *dst, const void *src, size_t len) {
    if (len == 0) {
        return 1;
    }
    __try {
        memmove(dst, src, len);
        return 1;
    } __except (GetExceptionCode() == 0xC0000005u || GetExceptionCode() == 0xC0000006u
                    ? EXCEPTION_EXECUTE_HANDLER
                    : EXCEPTION_CONTINUE_SEARCH) {
        return 0;
    }
}

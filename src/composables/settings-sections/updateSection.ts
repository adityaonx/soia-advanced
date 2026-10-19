import { computed, ref } from "vue";

export const useUpdateSection = () => {
    const hasAvailableUpdate = ref(false);
    const availableUpdateVersion = ref<string | null>(null);
    const updateButtonPhase = ref<"idle" | "checking" | "downloading" | "installing" | "failed">("idle");
    const shouldShowUpdateButton = computed(() => false);
    const isUpdateButtonDisabled = computed(() => true);
    const updateButtonText = computed(() => "Update");
    const isUpdateRetry = computed(() => false);
    const shouldShowUpdateStatus = computed(() => false);
    const updateStatusText = computed(() => "");
    const shouldShowUpdateHint = computed(() => false);
    const updateHintText = computed(() => "");

    const loadUpdateAvailability = async () => {};
    const setupUpdateAvailabilityListener = async () => {};
    const installUpdate = async () => {};
    const dispose = () => {};

    return {
        hasAvailableUpdate,
        availableUpdateVersion,
        updateButtonPhase,
        shouldShowUpdateButton,
        isUpdateButtonDisabled,
        updateButtonText,
        isUpdateRetry,
        shouldShowUpdateStatus,
        updateStatusText,
        shouldShowUpdateHint,
        updateHintText,
        loadUpdateAvailability,
        setupUpdateAvailabilityListener,
        installUpdate,
        dispose,
    };
};

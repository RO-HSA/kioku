const ESTIMATED_MANGA_TOTAL = 100;

export const getProgressValues = (progress: number, total: number) => {
  const clamp = (value: number) => Math.max(0, Math.min(100, value));
  const safeProgress = Math.max(0, progress);
  const safeTotal = total > 0 ? total : ESTIMATED_MANGA_TOTAL;
  const progressPercent = clamp((safeProgress / safeTotal) * 100);

  return {
    progressValue: progressPercent,
    bufferPercent: 100
  };
};

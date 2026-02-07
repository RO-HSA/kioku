import { TZDate } from '@date-fns/tz';
import { differenceInWeeks } from 'date-fns';

import { AnimeListBroadcast } from '@/services/backend/types';

export const estimateBroadcastedEpisodes = (
  total: number,
  startDate: string | null,
  broadcast: AnimeListBroadcast
): number => {
  if (!startDate || !broadcast.dayOfTheWeek || !broadcast.startTime) {
    return total;
  }

  const [year, month, day] = startDate.split('-').map(Number);
  const [hours, minutes] = broadcast.startTime.split(':').map(Number);

  const start = new TZDate(
    year,
    month - 1,
    day,
    hours,
    minutes,
    0,
    0,
    '+09:00'
  );
  const now = new TZDate(new Date(), '+09:00');

  const weeks = Math.max(0, differenceInWeeks(now, start));
  const estimated = now >= start ? weeks + 1 : 0;

  return total > 0 ? Math.min(estimated, total) : estimated;
};

export const getProgressValues = (
  progress: number,
  total: number,
  startDate: string | null,
  broadcast: AnimeListBroadcast
) => {
  const estimatedBuffer = estimateBroadcastedEpisodes(
    total,
    startDate,
    broadcast
  );

  if (estimatedBuffer <= 0) {
    return {
      progressValue: progress,
      bufferPercent: 0
    };
  }

  const clamp = (value: number) => Math.max(0, Math.min(100, value));
  const safeProgress = Math.max(0, progress);
  const safeEstimated = Math.max(0, estimatedBuffer);

  if (total > 0) {
    return {
      progressValue: clamp((safeProgress / total) * 100),
      bufferPercent: clamp((safeEstimated / total) * 100)
    };
  }

  if (safeEstimated > 1000) {
    return {
      progressValue: clamp((safeProgress / 1000) * 68),
      bufferPercent: clamp((safeEstimated / 1000) * 68)
    };
  }

  const progressPercent = clamp((safeProgress / 24) * 100);
  const unknownTotalPercent = clamp((safeEstimated / 24) * 100);
  return {
    progressValue: progressPercent,
    bufferPercent: unknownTotalPercent
  };
};

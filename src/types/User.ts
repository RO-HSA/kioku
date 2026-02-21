export interface Statistics {
  numItemsWatching: number;
  numItemsCompleted: number;
  numItemsOnHold: number;
  numItemsDropped: number;
  numItemsPlanToWatch: number;
  numItems: number;
  numDaysWatched: number;
  numDaysWatching: number;
  numDaysCompleted: number;
  numDaysOnHold: number;
  numDaysDropped: number;
  numDays: number;
  numEpisodes: number;
  numTimesRewatched: number;
  meanScore: number;
}

export interface IUser {
  id: number;
  username: string;
  profilePictureUrl: string;
  statistics: Statistics;
}

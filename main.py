import time
import psutil
import polars as pl

def count_observations(df):
    return len(df)

def sum_volume(df):
    return df["Volume"].sum()

def describe_with_polars(df):
    """Function which returns descriptive stats about input data using Polars."""
    return df.describe()


def main():
    start_time = time.time()
    memory_before = psutil.virtual_memory().used / (1024.0 ** 2)  # Convert bytes to MB
   
    spx = pl.read_csv("SPX.csv")
    results = describe_with_polars(spx)
    number_of_observations = count_observations(spx)
    total_volume = sum_volume(spx)
    
    end_time = time.time()
    memory_after = psutil.virtual_memory().used / (1024.0 ** 2)
    
    elapsed_time = end_time - start_time
    memory_used = memory_after - memory_before
    results = describe_with_polars("SPX.csv")

    print(results)


    print(f"There are {number_of_observations} days observed in the dataset.")
    print(f"Total of all volume traded: {total_volume}")
    print(f"This took {elapsed_time} seconds to complete")
    print(f"This used {memory_used} MB of memory to complete")
    return True

if __name__ == "__main__":
    main()

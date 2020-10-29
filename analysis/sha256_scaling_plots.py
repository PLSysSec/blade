import matplotlib.pyplot as plt
import json

def get_data(jsonfilename):
    """
    Returns a dict mapping blade_type to {dict mapping labels to lists of data}
    """
    with open(jsonfilename) as f:
        data = json.load(f)
    return data

def runtime_graph(sizes, runtimes_ns, outfilename, v1_1):
    """
    runtimes_ns: a dict mapping blade_type to lists of data
    v1_1: boolean, whether to include v1.1 protections
    """
    plt.figure(outfilename, figsize=(3.5,2.2))
    plt.rcParams['pdf.fonttype'] = 42  # truetype
    plt.rcParams['font.family'] = 'Times New Roman'
    plt.rcParams['font.size'] = '8'

    #plt.title("SHA256 runtimes, no v1.1 protections")
    plt.xlabel("Workload size (bytes)")
    plt.ylabel("Runtime (µs)")
    plt.xscale('log')
    plt.yscale('log')
    plt.xticks([])
    plt.xticks(
        [64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536],
        ["64", "128", "256", "512", "1KB", "2KB", "4KB", "8KB", "16KB", "32KB", "64KB"],
        fontsize=6
    )

    if v1_1:
        baseline_f_runtimes = runtimes_ns['Baseline-F with v1.1']
        blade_f_runtimes = runtimes_ns['Lfence with v1.1']
        baseline_s_runtimes = runtimes_ns['Baseline-S with v1.1']
        blade_s_runtimes = runtimes_ns['SLH with v1.1']
    else:
        baseline_f_runtimes = runtimes_ns['Baseline-F without v1.1']
        blade_f_runtimes = runtimes_ns['Lfence without v1.1']
        baseline_s_runtimes = runtimes_ns['Baseline-S without v1.1']
        blade_s_runtimes = runtimes_ns['SLH without v1.1']

    plt.plot(sizes, [rt / 1000 for rt in runtimes_ns['ref']], marker='^', linestyle='solid', linewidth=1, markersize=5, label="Ref")
    plt.plot(sizes, [rt / 1000 for rt in baseline_f_runtimes], marker='o', linestyle='solid', linewidth=1, markersize=3, label="Baseline-F")
    plt.plot(sizes, [rt / 1000 for rt in blade_f_runtimes], marker='+', linestyle='solid', linewidth=1, markersize=7, label="Blade-F")
    plt.plot(sizes, [rt / 1000 for rt in baseline_s_runtimes], marker='D', linestyle='solid', linewidth=1, markersize=3, label="Baseline-S")
    plt.plot(sizes, [rt / 1000 for rt in blade_s_runtimes], marker='x', linestyle='solid', linewidth=1, markersize=5, label="Blade-S")

    plt.legend()
    plt.tight_layout()
    plt.savefig(outfilename)
    plt.close(outfilename)

def runtime_per_byte_graph(sizes, runtimes_ns_per_byte, outfilename, v1_1):
    """
    runtimes_ns_per_byte: a dict mapping blade_type to lists of data
    v1_1: boolean, whether to include v1.1 protections
    """
    plt.figure(outfilename, figsize=(3.5,2.2))
    plt.rcParams['pdf.fonttype'] = 42  # truetype
    plt.rcParams['font.family'] = 'Times New Roman'
    plt.rcParams['font.size'] = '8'

    #plt.title("SHA256 runtimes (per-byte), no v1.1 protections")
    plt.xlabel("Workload size (bytes)")
    plt.ylabel("Per-byte runtime (ns/byte)")
    plt.xscale('log')
    plt.yscale('linear')
    plt.xticks([])
    plt.xticks(
        [64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536],
        ["64", "128", "256", "512", "1KB", "2KB", "4KB", "8KB", "16KB", "32KB", "64KB"],
        fontsize=6
    )

    if v1_1:
        baseline_f_runtimes = runtimes_ns_per_byte['Baseline-F with v1.1']
        blade_f_runtimes = runtimes_ns_per_byte['Lfence with v1.1']
        baseline_s_runtimes = runtimes_ns_per_byte['Baseline-S with v1.1']
        blade_s_runtimes = runtimes_ns_per_byte['SLH with v1.1']
    else:
        baseline_f_runtimes = runtimes_ns_per_byte['Baseline-F without v1.1']
        blade_f_runtimes = runtimes_ns_per_byte['Lfence without v1.1']
        baseline_s_runtimes = runtimes_ns_per_byte['Baseline-S without v1.1']
        blade_s_runtimes = runtimes_ns_per_byte['SLH without v1.1']

    plt.plot(sizes, runtimes_ns_per_byte['ref'], marker='^', linestyle='solid', linewidth=1, markersize=5, label="Ref")
    plt.plot(sizes, baseline_f_runtimes, marker='o', linestyle='solid', linewidth=1, markersize=3, label="Baseline-F")
    plt.plot(sizes, blade_f_runtimes, marker='+', linestyle='solid', linewidth=1, markersize=7, label="Blade-F")
    plt.plot(sizes, baseline_s_runtimes, marker='D', linestyle='solid', linewidth=1, markersize=3, label="Baseline-S")
    plt.plot(sizes, blade_s_runtimes, marker='x', linestyle='solid', linewidth=1, markersize=5, label="Blade-S")

    plt.legend()
    plt.tight_layout()
    plt.savefig(outfilename)
    plt.close(outfilename)

if __name__ == "__main__":
    import os
    data = get_data("./sha256_scaling.json")
    os.makedirs("./figures", exist_ok=True)
    runtime_graph(data['ref']['sizes'], { blade_type: data[blade_type]['runtimes_ns'] for blade_type in data.keys() }, "./figures/runtime_graph_no_v1_1.pdf", False)
    runtime_graph(data['ref']['sizes'], { blade_type: data[blade_type]['runtimes_ns'] for blade_type in data.keys() }, "./figures/runtime_graph_with_v1_1.pdf", True)
    runtime_per_byte_graph(data['ref']['sizes'], { blade_type: data[blade_type]['runtimes_ns_per_byte'] for blade_type in data.keys() }, "./figures/runtime_per_byte_graph_no_v1_1.pdf", False)
    runtime_per_byte_graph(data['ref']['sizes'], { blade_type: data[blade_type]['runtimes_ns_per_byte'] for blade_type in data.keys() }, "./figures/runtime_per_byte_graph_with_v1_1.pdf", True)
    print("Plots for scaling results created as analysis/figures/*.pdf")

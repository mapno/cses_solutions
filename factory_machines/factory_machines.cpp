#include <bits/stdc++.h>

using namespace std;

typedef pair<int, int> pii;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);

    int n, p, m;
    cin >> n >> p;

    priority_queue<pii, vector<pii>, greater<pii>> pq;

    for (int i = 0; i < n; i++) {
        cin >> m;
        pq.push(make_pair(m, m));
    }

    for (int pm = 0; pm != p;) {
        int s, mc;
        tie(s, mc) = pq.top();
        pq.pop();
        int m = p / s;
        s *= m;
        pm += m;
        pq.push(make_pair(s + mc, mc));
    }

    int b = 0;
    while (!pq.empty()) {
        auto p = pq.top();
        b = max(b, get<0>(p) - get<1>(p));
        pq.pop();
    }
    
    cout << b;
}
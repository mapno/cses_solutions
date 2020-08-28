#include <bits/stdc++.h>

using namespace std;

typedef pair<int, int> pii;
typedef tuple<int, int, int> tiii;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
 
    int n;
    cin >> n;
    vector<tiii> v;
    v.reserve(n);

    for (int i = 0; i < n; i++) {
        int a, d;
        cin >> a >> d;
        v.push_back(make_tuple(a, d, i));
    }
 
    sort(v.begin(), v.end(), [](const tiii & a, tiii & b) -> bool {
        return get<0>(a) < get<0>(b);
    });
 
    int k = 0;
	int o[n];
    priority_queue<pii, vector<pii>, greater<pii> > pq;

    for (auto c: v) {
        int a, d, i, room;
        tie(a, d, i) = c;
        if (pq.empty() || pq.top().first >= a) {
            room = ++k;
        } else {
            room = pq.top().second;
            pq.pop();
        }

        pq.push(make_pair(d, room));
        o[i] = room;
    }
 
    cout << pq.size() << endl;
    for (auto oo : o) {
        cout << oo << " ";
    }
}